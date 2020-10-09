#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use log::warn;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct UserDBLocal {
    source_files: Files,
    pub passwd_entries: Vec<crate::User>,
    pub shadow_entries: Vec<crate::Shadow>,
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
        let res = Self {
            source_files: Files {
                passwd: None,
                group: None,
                shadow: None,
            },
            passwd_entries: passwd_content
                .lines()
                .filter_map(|line| {
                    if line.len() > 5 {
                        println!("{}", line);
                        Some(
                            crate::User::new_from_string(line.to_owned())
                                .expect("failed to read lines"),
                        )
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
            shadow_entries: shadow_content
                .lines()
                .filter_map(|line| {
                    if line.len() > 5 {
                        Some(
                            crate::Shadow::new_from_string(line.to_owned())
                                .expect("Parsing failed"),
                        )
                    } else {
                        None
                    }
                })
                .collect(),
        };
        res
    }

    #[must_use]
    pub fn load_files(files: Files) -> Self {
        let passwd_file = File::open(
            files
                .group
                .clone()
                .expect("passwd file path cannot be None"),
        )
        .unwrap();
        let mut passwd_reader = BufReader::new(passwd_file);
        let mut my_passwd_lines = String::new();
        passwd_reader.read_to_string(&mut my_passwd_lines).unwrap();
        let group_file =
            File::open(files.group.clone().expect("group file path cannot be None")).unwrap();
        let mut group_reader = BufReader::new(group_file);
        let mut my_group_lines = String::new();
        group_reader.read_to_string(&mut my_group_lines).unwrap();
        let shadow_file = File::open(
            files
                .shadow
                .clone()
                .expect("shadow file path cannot be None"),
        )
        .expect("Failed to read the shadow file. Most of the time root permissions are needed");
        let mut shadow_reader = BufReader::new(shadow_file);
        let mut my_shadow_lines = String::new();
        shadow_reader.read_to_string(&mut my_shadow_lines).unwrap();

        Self {
            source_files: files,
            passwd_entries: string_to(&my_passwd_lines),
            group_entries: string_to(&my_group_lines),
            shadow_entries: string_to(&my_shadow_lines),
        }
    }
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
    let data = UserDBLocal::import_from_strings("testuser:x:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test", "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::", "teste:x:1002:test,teste");
    assert_eq!(
        data.passwd_entries.get(0).unwrap().get_username(),
        "testuser"
    )
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
