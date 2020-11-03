#![warn(
    clippy::all,
    //clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

pub mod files;
pub mod hashes;

use crate::api::UserRead;
use crate::{api::GroupRead, UserLibError};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct UserDBLocal {
    source_files: files::Files,
    source_hashes: hashes::Hashes, // to detect changes
    pub users: HashMap<String, crate::User>,
    pub groups: Vec<crate::Group>,
}

impl UserDBLocal {
    /// Import the database from strings
    #[must_use]
    pub fn import_from_strings(
        passwd_content: &str,
        shadow_content: &str,
        group_content: &str,
    ) -> Self {
        let shadow_entries: Vec<crate::Shadow> = string_to(&shadow_content);
        let mut users = user_vec_to_hashmap(string_to(&passwd_content));
        let groups = string_to(&group_content);
        shadow_to_users(&mut users, shadow_entries);
        let res = Self {
            source_files: files::Files {
                passwd: None,
                group: None,
                shadow: None,
            },
            users,
            groups,
            source_hashes: hashes::Hashes::new(&passwd_content, &shadow_content, &group_content),
        };
        res
    }

    /// Import the database from a [`Files`] struct
    #[must_use]
    pub fn load_files(files: files::Files) -> Result<Self, crate::UserLibError> {
        // Get the Strings for the files use an inner block to drop references after read.
        let (my_passwd_lines, my_shadow_lines, my_group_lines) = {
            let opened = files.lock_all_get();
            let (locked_p, locked_s, locked_g) = opened.expect("failed to lock files!");
            // read the files to strings
            let p = file_to_string(&locked_p.file)?;
            let s = file_to_string(&locked_s.file)?;
            let g = file_to_string(&locked_g.file)?;
            // return the strings to the outer scope and release the lock...
            (p, s, g)
        };

        let mut users = user_vec_to_hashmap(string_to(&my_passwd_lines));
        let passwds: Vec<crate::Shadow> = string_to(&my_shadow_lines);
        shadow_to_users(&mut users, passwds);
        Ok(Self {
            source_files: files,
            users,
            groups: string_to(&my_group_lines),
            source_hashes: hashes::Hashes::new(&my_passwd_lines, &my_shadow_lines, &my_group_lines),
        })
    }
    fn delete_from_passwd(
        user: &crate::User,
        passwd_file_content: String,
        locked_p: &mut files::LockedFileGuard,
    ) -> Result<(), UserLibError> {
        let modified_p = user.remove_in(&passwd_file_content);

        // write the new content to the file.
        let ncont = locked_p.replace_contents(modified_p);
        match ncont {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to write the passwd database: {}", e).into()),
        }
    }

    fn delete_from_shadow(
        user: &crate::User,
        shadow_file_content: String,
        locked_s: &mut files::LockedFileGuard,
    ) -> Result<(), UserLibError> {
        let shad = user.get_shadow();
        match shad {
            Some(shadow) => {
                let modified_s = shadow.remove_in(&shadow_file_content);
                let ncont = locked_s.replace_contents(modified_s);
                match ncont {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!(
                        "Error during write to the database. \
                    Please doublecheck as the shadowdatabase could be corrupted: {}",
                        e,
                    )
                    .into()),
                }
            }
            None => Ok(()),
        }
    }

    fn delete_from_group(
        group: &crate::Group,
        group_file_content: String,
        locked_g: &mut files::LockedFileGuard,
    ) -> Result<(), UserLibError> {
        let modified_g = group.remove_in(&group_file_content);
        let replace_result = locked_g.replace_contents(modified_g);
        match replace_result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error during write to the database. \
            Please doublecheck as the groupdatabase could be corrupted: {}",
                e,
            )
            .into()),
        }
    }

    fn delete_home(user: &crate::User) -> std::io::Result<()> {
        match user.get_home_dir() {
            Some(dir) => std::fs::remove_dir_all(dir),
            None => {
                error!("Failed to remove the home directory! As the user did not have one.");
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Failed to remove the home directory! As the user did not have one.",
                ))
            }
        }
    }

    fn get_group_pos_by_id(&self, id: u32) -> Option<(&crate::Group, usize)> {
        for (i, group) in self.groups.iter().enumerate() {
            if group.get_gid()? == id {
                return Some((group, i));
            }
        }
        None
    }
}

use crate::api::{DeleteHome, DeletePrimaryGroup, NewUserArgs, UserDBRead, UserDBWrite};
impl UserDBWrite for UserDBLocal {
    fn delete_user(&mut self, args: NewUserArgs) -> Result<crate::User, UserLibError> {
        // try to get the user from the database
        let user_opt = self.get_user_by_name(args.username);
        let user = match user_opt {
            Some(user) => user,
            None => {
                return Err(UserLibError::NotFound);
            }
        };

        if self.source_files.is_virtual() {
            warn!("There are no associated files working in dummy mode!");
            let res = self.users.remove(args.username);
            match res {
                Some(u) => Ok(u),
                None => Err(UserLibError::NotFound), // should not happen anymore as existence is checked.
            }
        } else {
            let opened = self.source_files.lock_all_get();
            let (mut locked_p, mut locked_s, mut locked_g) = opened.expect("failed to lock files!");

            // read the files to strings
            let passwd_file_content = file_to_string(&locked_p.file)?;
            let shadow_file_content = file_to_string(&locked_s.file)?;
            let group_file_content = file_to_string(&locked_g.file)?;

            let src = &self.source_hashes;
            if src.passwd.has_changed(&passwd_file_content)
                | src.shadow.has_changed(&shadow_file_content)
            {
                error!("The source files have changed. Deleting the user could corrupt the userdatabase. Aborting!");
                Err(format!("The userdatabase has been changed {}", args.username).into())
            } else {
                UserDBLocal::delete_from_passwd(user, passwd_file_content, &mut locked_p)?;
                UserDBLocal::delete_from_shadow(user, shadow_file_content, &mut locked_s)?;
                if args.delete_home == DeleteHome::Delete {
                    UserDBLocal::delete_home(user)?;
                }
                let group = self.get_group_pos_by_id(user.get_gid());
                match group {
                    Some((group, id)) => {
                        if group
                            .get_member_names()
                            .expect("groups have to have members")
                            .len()
                            == 1
                        {
                            UserDBLocal::delete_from_group(
                                group,
                                group_file_content,
                                &mut locked_g,
                            )?;
                            let _gres = self.groups.remove(id);
                        } else {
                            warn!(
                                "The primary group {} was not empty and is thus not removed.",
                                group.get_groupname().unwrap()
                            );
                        }
                    }
                    None => warn!(
                        "The users primary group could not be found {}",
                        user.get_gid()
                    ),
                }
                // Remove the user from the memory database(HashMap)
                let res = self.users.remove(args.username);
                match res {
                    Some(u) => Ok(u),
                    None => Err("Failed to remove the user from the internal HashMap".into()),
                }
            }
        }
    }

    fn new_user(
        &mut self, /*
                   username: String,
                   enc_password: String,
                   uid: u32,
                   gid: u32,
                   full_name: String,
                   room: String,
                   phone_work: String,
                   phone_home: String,
                   other: Option<Vec<String>>,
                   home_dir: String,
                   shell_path: String,*/
    ) -> Result<&crate::User, crate::UserLibError> {
        /*if self.users.contains_key(&username) {
            Err(format!(
                "The username {} already exists! Aborting!",
                username
            )
            .into())
        } else {
            let pwd = if self.source_files.shadow.is_none(){
                crate::Password::Encrypted(crate::EncryptedPassword{});
            }
            else{
                crate::Password::Shadow(crate::Shadow{})
            }
            self.users.insert(
                username,
                crate::User {
                    username: crate::Username { username },
                    password:,
                    uid: crate::Uid{uid},
                    gid:crate::Gid{gid},
                    gecos: crate::Gecos{},
                    home_dir:crate::HomeDir{dir: home_dir},
                    shell_path: crate::ShellPath{shell: shell_path},
                },
            )
        }*/
        todo!()
    }

    fn delete_group(&mut self, _group: &crate::Group) -> Result<(), crate::UserLibError> {
        todo!()
    }

    fn new_group(&mut self) -> Result<&crate::Group, crate::UserLibError> {
        todo!()
    }
}

impl UserDBRead for UserDBLocal {
    fn get_all_users(&self) -> Vec<&crate::User> {
        let mut res: Vec<&crate::User> = self.users.iter().map(|(_, x)| x).collect();
        res.sort();
        res
    }

    fn get_user_by_name(&self, name: &str) -> Option<&crate::User> {
        self.users.get(name)
    }

    fn get_user_by_id(&self, uid: u32) -> Option<&crate::User> {
        // could probably be more efficient - on the other hand its no problem to loop a thousand users.
        for (_, user) in self.users.iter() {
            if user.get_uid() == uid {
                return Some(&user);
            }
        }
        None
    }

    fn get_all_groups(&self) -> Vec<&crate::Group> {
        self.groups.iter().collect()
    }

    fn get_group_by_name(&self, name: &str) -> Option<&crate::Group> {
        for group in self.groups.iter() {
            if group.get_groupname()? == name {
                return Some(group);
            }
        }
        None
    }

    fn get_group_by_id(&self, id: u32) -> Option<&crate::Group> {
        for group in self.groups.iter() {
            if group.get_gid()? == id {
                return Some(group);
            }
        }
        None
    }
}

use crate::api::UserDBValidation;
impl UserDBValidation for UserDBLocal {
    fn is_uid_valid_and_free(&self, uid: u32) -> bool {
        warn!("No valid check, only free check");
        let free = self.users.iter().all(|(_, u)| u.get_uid() != uid);
        free
    }

    fn is_username_valid_and_free(&self, name: &str) -> bool {
        let valid = crate::user::passwd_fields::is_username_valid(name);
        let free = self.get_user_by_name(name).is_none();
        valid && free
    }

    fn is_gid_valid_and_free(&self, gid: u32) -> bool {
        warn!("No valid check, only free check");
        self.groups.iter().all(|x| x.get_gid().unwrap() != gid)
    }

    fn is_groupname_valid_and_free(&self, name: &str) -> bool {
        let valid = crate::group::is_groupname_valid(name);
        let free = self
            .groups
            .iter()
            .all(|x| x.get_groupname().unwrap() != name);
        valid && free
    }
}

/// Parse a file to a string
fn file_to_string(file: &File) -> Result<String, crate::UserLibError> {
    let mut reader = BufReader::new(file);
    let mut lines = String::new();
    let res = reader.read_to_string(&mut lines);
    match res {
        Ok(_) => Ok(lines),
        Err(e) => Err(format!("failed to read the file: {:?}", e).into()),
    }
}

/// Merge the Shadow passwords into the users
fn shadow_to_users(
    users: &mut HashMap<String, crate::User>,
    shadow: Vec<crate::Shadow>,
) -> &mut HashMap<String, crate::User> {
    for pass in shadow {
        let user = users
            .get_mut(pass.get_username())
            .expect(&format!("the user {} does not exist", pass.get_username()));
        user.password = crate::Password::Shadow(pass);
    }
    users
}

/// Convert a `Vec<crate::User>` to a `HashMap<String, crate::User>` where the username is used as key
fn user_vec_to_hashmap(users: Vec<crate::User>) -> HashMap<String, crate::User> {
    users
        .into_iter()
        .map(|x| {
            (
                x.get_username()
                    .expect("An empty username is not supported")
                    .to_owned(),
                x,
            )
        })
        .collect()
}

/// Try to parse a String into some Object
///
/// # Errors
/// if the parsing failed a [`UserLibError::Message`](crate::userlib_error::UserLibError::Message) is returned containing a more detailed error message.
pub trait NewFromString {
    fn new_from_string(line: String, position: u32) -> Result<Self, crate::UserLibError>
    where
        Self: Sized;
}

/// A generic function that parses a string line by line and creates the appropriate `Vec<T>` requested by the type system.
fn string_to<T>(source: &str) -> Vec<T>
where
    T: NewFromString,
{
    source
        .lines()
        .enumerate()
        .filter_map(|(n, line)| {
            if line.len() > 5 {
                Some(T::new_from_string(line.to_owned(), n as u32).expect("failed to read lines"))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_creator_user_db_local() {
    let data = UserDBLocal::import_from_strings("test:x:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test", "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::", "teste:x:1002:test,test");
    assert_eq!(
        data.users.get("test").unwrap().get_username().unwrap(),
        "test"
    )
}

#[test]
fn test_parsing_local_database() {
    use std::path::PathBuf;
    // Parse the worldreadable user database ignore the shadow database as this would require root privileges.
    let pwdfile = File::open(PathBuf::from("/etc/passwd")).unwrap();
    let grpfile = File::open(PathBuf::from("/etc/group")).unwrap();
    let my_passwd_lines = file_to_string(&pwdfile).unwrap();
    let my_group_lines = file_to_string(&grpfile).unwrap();
    let data = UserDBLocal::import_from_strings(&my_passwd_lines, "", &my_group_lines);
    assert_eq!(data.groups.get(0).unwrap().get_groupname().unwrap(), "root");
}

#[test]
fn test_user_db_read_implementation() {
    use std::path::PathBuf;
    let pwdfile = File::open(PathBuf::from("/etc/passwd")).unwrap();
    let grpfile = File::open(PathBuf::from("/etc/group")).unwrap();
    let pass = file_to_string(&pwdfile).unwrap();
    let group = file_to_string(&grpfile).unwrap();
    let data = UserDBLocal::import_from_strings(&pass, "", &group);
    // Usually there are more than 10 users
    assert!(data.get_all_users().len() > 10);
    assert!(data.get_user_by_name("root").is_some());
    assert_eq!(data.get_user_by_name("root").unwrap().get_uid(), 0);
    assert_eq!(
        data.get_user_by_id(0).unwrap().get_username().unwrap(),
        "root"
    );
    assert!(data.get_all_groups().len() > 10);
    assert!(data.get_group_by_name("root").is_some());
    assert_eq!(
        data.get_group_by_name("root").unwrap().get_gid().unwrap(),
        0
    );
    assert_eq!(
        data.get_group_by_id(0).unwrap().get_groupname().unwrap(),
        "root"
    );
    assert!(data.get_user_by_name("norealnameforsure").is_none());
    assert!(data.get_group_by_name("norealgroupforsure").is_none());
}

#[test]
fn test_user_db_write_implementation() {
    use crate::api::NewUserArgs;
    let mut data = UserDBLocal::import_from_strings("test:x:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test", "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::", "teste:x:1002:test,test");
    let user = "test";

    assert_eq!(data.get_all_users().len(), 1);
    assert!(data
        .delete_user(NewUserArgs::builder().username(&user).build().unwrap())
        .is_ok());
    assert!(data
        .delete_user(NewUserArgs::builder().username(&user).build().unwrap())
        .is_err());
    assert_eq!(data.get_all_users().len(), 0);
}
