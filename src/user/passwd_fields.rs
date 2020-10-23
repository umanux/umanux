#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use log::warn;
use regex::Regex;

use crate::UserLibError;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// The username of the current user
///
/// When done the validity will automatically be checked in the `trait TryFrom`.
///
/// In the future some extra fields might be added.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Username {
    /// The username value
    pub(crate) username: String,
}

impl Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.username,)
    }
}

impl TryFrom<String> for Username {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        if is_username_valid(&source) {
            Ok(Self { username: source })
        } else if source == "Debian-exim" {
            warn!("username {} is not a valid username. This might cause problems. (It is default in Debian and Ubuntu)", source);
            Ok(Self { username: source })
        } else {
            Err(UserLibError::Message(format!(
                "Invalid username {}",
                source
            )))
        }
    }
}

pub(crate) fn is_username_valid(name: &str) -> bool {
    lazy_static! {
        static ref USERVALIDATION: Regex =
            Regex::new("^[a-z_]([a-z0-9_\\-]{0,31}|[a-z0-9_\\-]{0,30}\\$)$").unwrap();
    }
    USERVALIDATION.is_match(name)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Password {
    Encrypted(crate::EncryptedPassword),
    Shadow(crate::Shadow),
    Disabled,
}

impl Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encrypted(EncryptedPassword { password }) => write!(f, "{}", password,),
            Self::Shadow(_) => write!(f, "x"),
            Self::Disabled => write!(f, "x"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct EncryptedPassword {
    pub(in crate::user) password: String,
}

impl Display for EncryptedPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.password,)
    }
}

impl TryFrom<String> for EncryptedPassword {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        if source == "x" {
            warn!("password from shadow not loaded!")
        } else {
            warn!("Password field has an unexpected value")
        };
        Ok(Self { password: source })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Uid {
    pub(in crate::user) uid: u32,
}

impl Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uid,)
    }
}

impl TryFrom<String> for Uid {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            uid: source.parse::<u32>().unwrap(),
        })
    }
}

impl Uid {
    #[must_use]
    pub const fn is_system_uid(&self) -> bool {
        // since it is a u32  it cannot be smaller than 0
        self.uid < 1000
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Gid {
    pub(in crate::user) gid: u32,
}

impl Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.gid,)
    }
}

impl TryFrom<String> for Gid {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            gid: source.parse::<u32>().unwrap(),
        })
    }
}

impl Gid {
    #[must_use]
    pub const fn is_system_gid(&self) -> bool {
        // since it is a u32  it cannot be smaller than 0
        self.gid < 1000
    }

    pub const fn get_gid(&self) -> u32 {
        self.gid
    }
}

/// The home directory of a user
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HomeDir {
    pub(in crate::user) dir: String,
}

impl Display for HomeDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dir,)
    }
}

impl TryFrom<String> for HomeDir {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        Ok(Self { dir: source })
    }
}

/// The path to the Shell binary
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ShellPath {
    pub(in crate::user) shell: String,
}

impl Display for ShellPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shell,)
    }
}

impl TryFrom<String> for ShellPath {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        Ok(Self { shell: source })
    }
}

// Tests ----------------------------------------------------------------------

#[test]
fn test_username_validation() {
    // Failing tests
    let umlauts: Result<Username, UserLibError> = Username::try_from("täst".to_owned()); // umlauts
    assert_eq!(
        Err(UserLibError::Message("Invalid username täst".into())),
        umlauts
    );
    let number_first = Username::try_from("11elf".to_owned()); // numbers first
    assert_eq!(
        Err(UserLibError::Message("Invalid username 11elf".into())),
        number_first
    );
    let slashes = Username::try_from("test/name".to_owned()); // slashes in the name
    assert_eq!(
        Err(UserLibError::Message("Invalid username test/name".into())),
        slashes
    );
    let long = Username::try_from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()); // maximum size 32 letters
    assert_eq!(
        Err(UserLibError::Message(
            "Invalid username aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()
        )),
        long
    );
    // Working tests
    let ubuntu_exception = Username::try_from("Debian-exim".to_owned()); // for some reason ubuntu and debian have a capital user.
    assert_eq!(ubuntu_exception.unwrap().username, "Debian-exim");
    let single = Username::try_from("t".to_owned()); // single characters are ok
    assert_eq!(single.unwrap().username, "t");
    let normal = Username::try_from("superman".to_owned()); // regular username
    assert_eq!(normal.unwrap().username, "superman");
    let normal = Username::try_from("anna3pete".to_owned()); // regular username containing a number
    assert_eq!(normal.unwrap().username, "anna3pete");
    let normal = Username::try_from("enya$".to_owned()); // regular username ending in a $
    assert_eq!(normal.unwrap().username, "enya$");
}

#[test]
fn test_guid_system_user() {
    // Check uids of system users.
    let values = vec![
        ("999".to_owned(), true),
        ("0".to_owned(), true),
        ("1000".to_owned(), false),
    ];
    for val in values {
        assert_eq!(Uid::try_from(val.0.clone()).unwrap().is_system_uid(), val.1);
        assert_eq!(Gid::try_from(val.0.clone()).unwrap().is_system_gid(), val.1);
    }
}
