#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use log::warn;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct UserDBLocal {
    source_files: Files,
    pub users: HashMap<String, crate::User>,
    pub group_entries: Vec<crate::Group>,
}

pub struct Files {
    passwd: Option<PathBuf>,
    shadow: Option<PathBuf>,
    group: Option<PathBuf>,
}

impl Default for Files {
    fn default() -> Self {
        Self {
            passwd: Some(PathBuf::from("/etc/passwd")),
            shadow: Some(PathBuf::from("/etc/shadow")),
            group: Some(PathBuf::from("/etc/group")),
        }
    }
}

impl UserDBLocal {
    #[must_use]
    pub fn import_from_strings(
        passwd_content: &str,
        shadow_content: &str,
        group_content: &str,
    ) -> Self {
        let shadow_entries: Vec<crate::Shadow> = shadow_content
            .lines()
            .filter_map(|line| {
                if line.len() > 5 {
                    Some(crate::Shadow::new_from_string(line.to_owned()).expect("Parsing failed"))
                } else {
                    None
                }
            })
            .collect();
        let mut res = Self {
            source_files: Files {
                passwd: None,
                group: None,
                shadow: None,
            },
            users: passwd_content
                .lines()
                .filter_map(|line| {
                    if line.len() > 5 {
                        println!("{}", line);
                        let user = crate::User::new_from_string(line.to_owned())
                            .expect("failed to read lines");
                        Some((user.get_username().to_owned(), user))
                    } else {
                        None
                    }
                })
                .collect(),
            group_entries: group_content
                .lines()
                .filter_map(|line| {
                    if line.len() > 5 {
                        Some(
                            crate::Group::new_from_string(line.to_owned()).expect("Parsing failed"),
                        )
                    } else {
                        None
                    }
                })
                .collect(),
        };
        for shadow in shadow_entries {
            let user = res.users.get_mut(shadow.get_username()).expect(&format!(
                "the user {} does not exist",
                shadow.get_username()
            ));
            user.password = crate::Password::Shadow(shadow);
        }
        res
    }

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
            group_entries: string_to(&my_group_lines),
        }
    }
}

fn file_to_string(path: Option<&PathBuf>) -> String {
    let file = File::open(path.expect("Path cannot be None".into()))
        .expect("Failed to read the file. Most of the time root permissions are needed".into());
    let mut reader = BufReader::new(file);
    let mut lines = String::new();
    reader.read_to_string(&mut lines).unwrap();
    lines
}

/// Merge the Shadow passwords into the users.
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

/// Convert a `Vec<crate::User>` to a `HashMap<String, crate::User>` where the username is used as key.
fn user_vec_to_hashmap(users: Vec<crate::User>) -> HashMap<String, crate::User> {
    users
        .into_iter()
        .map(|x| (x.get_username().to_owned(), x))
        .collect()
}

/// Try to parse a String into some Object
///
/// # Errors
/// if the parsing failed a [`UserLibError::Message`] is returned containing a more detailed error message.
pub trait NewFromString {
    fn new_from_string(line: String) -> Result<Self, crate::UserLibError>
    where
        Self: Sized;
}

fn string_to<T>(source: &str) -> Vec<T>
where
    T: NewFromString,
{
    source
        .lines()
        .filter_map(|line| {
            if line.len() > 5 {
                println!("{}", line);
                Some(T::new_from_string(line.to_owned()).expect("failed to read lines"))
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_creator_user_db_local() {
    let data = UserDBLocal::import_from_strings("test:x:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test", "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::", "teste:x:1002:test,test");
    assert_eq!(data.users.get("test").unwrap().get_username(), "test")
}

#[test]
fn test_parsing_local_database() {
    use std::fs::File;
    use std::io::{BufReader, Read};
    let passwd_file = File::open("/etc/passwd").unwrap();
    let mut passwd_reader = BufReader::new(passwd_file);
    let mut my_passwd_lines = "".to_string();
    passwd_reader.read_to_string(&mut my_passwd_lines).unwrap();
    let group_file = File::open("/etc/group").unwrap();
    let mut group_reader = BufReader::new(group_file);
    let mut my_group_lines = "".to_string();
    group_reader.read_to_string(&mut my_group_lines).unwrap();
    let data = UserDBLocal::import_from_strings(&my_passwd_lines, "", &my_group_lines);
    assert_eq!(data.group_entries.get(0).unwrap().get_groupname(), "root");
}
