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
        if is_groupname_valid(&source) {
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

pub(crate) fn is_groupname_valid(name: &str) -> bool {
    // for now just use the username validation.
    crate::user::passwd_fields::is_username_valid(name)
}

/// A record(line) in the user database `/etc/shadow` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    groupname: Groupname,                 /* Username.  */
    pub(crate) password: crate::Password, /* Usually not used (disabled with x) */
    gid: crate::Gid,                      /* Group ID.  */
    members: Vec<crate::Username>,        /* Real name.  */
}

use crate::api::GroupRead;
impl GroupRead for Group {
    #[must_use]
    fn get_groupname(&self) -> Option<&str> {
        Some(&self.groupname.groupname)
    }
    #[must_use]
    fn get_member_names(&self) -> Option<Vec<&str>> {
        let mut r: Vec<&str> = Vec::new();
        for u in self.members.iter() {
            r.push(&u.username);
        }
        Some(r)
    }

    fn get_gid(&self) -> Option<u32> {
        Some(self.gid.get_gid())
    }

    fn get_encrypted_password(&self) -> Option<&str> {
        todo!()
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
    /// Parse a line formatted like one in `/etc/group` and construct a matching [`Group`] instance
    ///
    /// # Example
    /// ```
    /// use crate::adduser::api::GroupRead;
    /// use adduser::NewFromString;
    /// let grp = adduser::Group::new_from_string(
    ///     "teste:x:1002:test,teste".to_owned()
    /// ).unwrap();
    /// assert_eq!(grp.get_groupname().unwrap(), "teste");
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
    assert_eq!(line2.get_groupname().unwrap(), "teste");
}
#[test]
fn test_root_group() {
    let line = "root:x:0:";
    let line2 = Group::new_from_string(line.to_owned()).unwrap();
    assert_eq!(line2.get_groupname().unwrap(), "root");
}