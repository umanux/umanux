#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use crate::userlib::NewFromString;
use log::warn;
use regex::Regex;

use crate::userlib_error::UserLibError;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Groupname {
    groupname: String,
}

impl Display for Groupname {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.groupname,)
    }
}

impl TryFrom<String> for Groupname {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        lazy_static! {
            static ref USERVALIDATION: Regex =
                Regex::new("^[a-z_]([a-z0-9_\\-]{0,31}|[a-z0-9_\\-]{0,30}\\$)$").unwrap();
        }
        if USERVALIDATION.is_match(&source) {
            Ok(Self { groupname: source })
        } else if source == "Debian-exim" {
            warn!("username {} is not a valid username. This might cause problems. (It is default in Debian and Ubuntu)", source);
            Ok(Self { groupname: source })
        } else {
            Err(UserLibError::Message(format!(
                "Invalid groupname -{}-",
                source
            )))
        }
    }
}

/// A record(line) in the user database `/etc/shadow` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    groupname: Groupname,                 /* Username.  */
    pub(crate) password: crate::Password, /* Usually not used (disabled with x) */
    gid: crate::Gid,                      /* Group ID.  */
    members: Vec<crate::Username>,        /* Real name.  */
}

impl Group {
    #[must_use]
    pub fn get_groupname(&self) -> &str {
        &self.groupname.groupname
    }
    #[must_use]
    pub const fn get_members(&self) -> &Vec<crate::Username> {
        &self.members
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}:{}:{}:{}",
            self.groupname,
            self.password,
            self.gid,
            self.members
                .iter()
                .map(|mem| format!("{}", mem))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl NewFromString for Group {
    /// Parse a line formatted like one in `/etc/shadow` and construct a matching `Shadow` instance
    ///
    /// # Example
    /// ```
    /// /*let shad = adduser::shadow::Shadow::new_from_string(
    ///     "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::"
    /// ).unwrap();
    /// assert_eq!(shad.get_username(), "test");*/
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a `UserLibError::Message` containing some information as to why the function failed.
    fn new_from_string(line: String) -> Result<Self, UserLibError> {
        println!("{}", &line);
        let elements: Vec<String> = line.split(':').map(ToString::to_string).collect();
        if elements.len() == 4 {
            Ok(Self {
                groupname: Groupname::try_from(elements.get(0).unwrap().to_string())?,
                password: crate::Password::Disabled,
                gid: crate::Gid::try_from(elements.get(2).unwrap().to_string())?,
                members: parse_members_list(elements.get(3).unwrap()),
            })
        } else {
            Err(UserLibError::Message(format!(
                "Failed to parse: not enough elements ({}): {:?}",
                elements.len(),
                elements
            )))
        }
    }
}

fn parse_members_list(source: &str) -> Vec<crate::Username> {
    let mut res = vec![];
    for mem in source.split(',').filter_map(|x| {
        if x.is_empty() {
            None
        } else {
            Some(x.to_string())
        }
    }) {
        res.push(crate::Username::try_from(mem).expect("failed to parse username"));
    }
    res
}

#[test]
fn test_parse_and_back_identity() {
    let line = "teste:x:1002:test,teste";
    let line2 = Group::new_from_string(line.to_owned()).unwrap();
    assert_eq!(format!("{}", line2), line);
}

#[test]
fn test_groupname() {
    let line = "teste:x:1002:test,teste";
    let line2 = Group::new_from_string(line.to_owned()).unwrap();
    assert_eq!(line2.get_groupname(), "teste");
}
#[test]
fn test_root_group() {
    let line = "root:x:0:";
    let line2 = Group::new_from_string(line.to_owned()).unwrap();
    assert_eq!(line2.get_groupname(), "root");
}
