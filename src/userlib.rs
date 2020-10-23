#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use crate::api::GroupRead;
use crate::api::UserRead;
use log::warn;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct UserDBLocal {
    source_files: Files,
    pub users: HashMap<String, crate::User>,
    pub groups: Vec<crate::Group>,
}

pub struct Files {
    passwd: Option<PathBuf>,
    shadow: Option<PathBuf>,
    group: Option<PathBuf>,
}

impl Default for Files {
    /// use the default Linux `/etc/` paths
    fn default() -> Self {
        Self {
            passwd: Some(PathBuf::from("/etc/passwd")),
            shadow: Some(PathBuf::from("/etc/shadow")),
            group: Some(PathBuf::from("/etc/group")),
        }
    }
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
            source_files: Files {
                passwd: None,
                group: None,
                shadow: None,
            },
            users,
            groups,
        };
        res
    }

    /// Import the database from a [`Files`] struct
    #[must_use]
    pub fn load_files(files: Files) -> Self {
        let my_passwd_lines = file_to_string(files.passwd.as_ref());
        let my_group_lines = file_to_string(files.group.as_ref());
        let my_shadow_lines = file_to_string(files.shadow.as_ref());

        let mut users = user_vec_to_hashmap(string_to(&my_passwd_lines));
        let passwds: Vec<crate::Shadow> = string_to(&my_shadow_lines);
        shadow_to_users(&mut users, passwds);
        Self {
            source_files: files,
            users,
            groups: string_to(&my_group_lines),
        }
    }
}

use crate::api::UserDBWrite;
impl UserDBWrite for UserDBLocal {
    fn delete_user(&mut self, user: &str) -> Result<crate::User, crate::UserLibError> {
        let res = self.users.remove(user);
        match res {
            Some(user) => Ok(user),
            None => Err(format!("Failed to delete the user {}", user).into()),
        }
    }

    fn new_user(
        &mut self,
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
        shell_path: String,
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

    fn delete_group(&mut self, group: &crate::Group) -> Result<(), crate::UserLibError> {
        todo!()
    }

    fn new_group(&mut self) -> Result<&crate::Group, crate::UserLibError> {
        todo!()
    }
}

use crate::api::UserDBRead;
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
            if group.get_groupname().unwrap() == name {
                return Some(group);
            }
        }
        None
    }

    fn get_group_by_id(&self, id: u32) -> Option<&crate::Group> {
        for group in self.groups.iter() {
            if group.get_gid().unwrap() == id {
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
fn file_to_string(path: Option<&PathBuf>) -> String {
    let file = File::open(path.expect("Path cannot be None".into()))
        .expect("Failed to read the file. Most of the time root permissions are needed".into());
    let mut reader = BufReader::new(file);
    let mut lines = String::new();
    reader.read_to_string(&mut lines).unwrap();
    lines
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
    // Parse the worldreadable user database ignore the shadow database as this would require root privileges.
    let my_passwd_lines = file_to_string(Some(&PathBuf::from("/etc/passwd")));
    let my_group_lines = file_to_string(Some(&PathBuf::from("/etc/group")));
    let data = UserDBLocal::import_from_strings(&my_passwd_lines, "", &my_group_lines);
    assert_eq!(data.groups.get(0).unwrap().get_groupname().unwrap(), "root");
}

#[test]
fn test_user_db_read_implementation() {
    let pass = file_to_string(Some(&PathBuf::from("/etc/passwd")));
    let group = file_to_string(Some(&PathBuf::from("/etc/group")));
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
    let mut data = UserDBLocal::import_from_strings("test:x:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test", "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::", "teste:x:1002:test,test");
    let user = "test";

    assert_eq!(data.get_all_users().len(), 1);
    assert!(data.delete_user(&user).is_ok());
    assert!(data.delete_user(&user).is_err());
    assert_eq!(data.get_all_users().len(), 0);
}
