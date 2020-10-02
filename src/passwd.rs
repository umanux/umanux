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

use crate::userlib_error::UserLibError;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// The username of the current user
///
/// When done the validity will automatically be checked in the `trait TryFrom`.
///
/// In the future some extra fields might be added.
#[derive(Debug, PartialEq, Eq)]
pub struct Username<'a> {
    /// The username value
    username: &'a str,
}

impl Display for Username<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.username,)
    }
}

impl<'a> TryFrom<&'a str> for Username<'a> {
    type Error = UserLibError;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        lazy_static! {
            static ref USERVALIDATION: Regex =
                Regex::new("^[a-z_]([a-z0-9_\\-]{0,31}|[a-z0-9_\\-]{0,30}\\$)$").unwrap();
        }
        if USERVALIDATION.is_match(source) {
            Ok(Self { username: source })
        } else if source == "Debian-exim" {
            //warn!("username {} is not a valid username. This might cause problems. (It is default in Debian and Ubuntu)", source);
            Ok(Self { username: source })
        } else {
            Err(UserLibError::Message(format!(
                "Invalid username {}",
                source
            )))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Password<'a> {
    password: &'a str,
}

impl Display for Password<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.password,)
    }
}

impl<'a> TryFrom<&'a str> for Password<'a> {
    type Error = UserLibError;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        if source == "x" {
            warn!("password from shadow not loaded!")
        } else {
            warn!("Password field has an unexpected value")
        };
        Ok(Self { password: source })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Uid {
    uid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gid {
    gid: u32,
}

/// The gecos field of a user.
///
/// In the `/etc/passwd` file this field is a `,` sepparated list of items.
/// The first 4 values are more or less standardised to be full name, room, phone at work and phone at home. After that there can be some extra fields often containing the emailadress and even additional information.
///
/// This enum represents the first 4 values by name and adds the other values to a list of strings [`Gecos::Detail`]. If only one field is found and no `,` at all this value is used as a human readable comment [`Gecos::Simple`].
#[derive(Debug, PartialEq, Eq)]
pub enum Gecos<'a> {
    Detail {
        full_name: &'a str,
        room: &'a str,
        phone_work: &'a str,
        phone_home: &'a str,
        other: Option<Vec<&'a str>>,
    },
    Simple {
        comment: &'a str,
    },
}

impl<'a> Gecos<'a> {
    #[must_use]
    pub const fn get_comment(&'a self) -> Option<&'a str> {
        match *self {
            Gecos::Simple { comment, .. } => Some(comment),
            Gecos::Detail { .. } => None,
        }
    }
    #[must_use]
    pub const fn get_full_name(&'a self) -> Option<&'a str> {
        match *self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { full_name, .. } => {
                if full_name.is_empty() {
                    None
                } else {
                    Some(full_name)
                }
            }
        }
    }
    #[must_use]
    pub const fn get_room(&'a self) -> Option<&'a str> {
        match *self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { room, .. } => {
                if room.is_empty() {
                    None
                } else {
                    Some(room)
                }
            }
        }
    }
    #[must_use]
    pub const fn get_phone_work(&'a self) -> Option<&'a str> {
        match *self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { phone_work, .. } => {
                if phone_work.is_empty() {
                    None
                } else {
                    Some(phone_work)
                }
            }
        }
    }
    #[must_use]
    pub const fn get_phone_home(&'a self) -> Option<&'a str> {
        match *self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { phone_home, .. } => {
                if phone_home.is_empty() {
                    None
                } else {
                    Some(phone_home)
                }
            }
        }
    }
    #[must_use]
    pub const fn get_other(&'a self) -> Option<&Vec<&'a str>> {
        match self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { other, .. } => match other {
                None => None,
                Some(comments) => Some(comments),
            },
        }
    }
}

impl Display for Gecos<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Gecos::Simple { comment } => write!(f, "{}", comment),
            Gecos::Detail {
                full_name,
                room,
                phone_work,
                phone_home,
                other,
            } => write!(
                f,
                "{},{},{},{}{}",
                full_name,
                room,
                phone_work,
                phone_home,
                match other {
                    None => "".to_string(),
                    Some(cont) => format!(",{}", cont.join(",")),
                }
            ),
        }
    }
}

impl<'a> TryFrom<&'a str> for Gecos<'a> {
    type Error = UserLibError;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        let vals: Vec<&str> = source.split(',').collect();
        if vals.len() > 3 {
            Ok(Gecos::Detail {
                full_name: vals[0],
                room: vals[1],
                phone_work: vals[2],
                phone_home: vals[3],
                other: if vals.len() == 4 {
                    None
                } else {
                    Some(vals[4..].to_vec())
                },
            })
        } else if vals.len() == 1 {
            Ok(Gecos::Simple {
                comment: vals.get(0).unwrap(),
            })
        } else {
            panic!(format!("Could not parse this string: {}", source))
        }
    }
}
/// The home directory of a user
#[derive(Debug, PartialEq, Eq)]
pub struct HomeDir<'a> {
    dir: &'a str,
}

/// The path to the Shell binary
#[derive(Debug, PartialEq, Eq)]
pub struct ShellPath<'a> {
    shell: &'a str,
}

/// A record(line) in the user database `/etc/passwd` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct Passwd<'a> {
    username: Username<'a>,    /* Username.  */
    password: Password<'a>, /* Hashed passphrase, if shadow database not in use (see shadow.h).  */
    uid: Uid,               /* User ID.  */
    gid: Gid,               /* Group ID.  */
    gecos: Gecos<'a>,       /* Real name.  */
    home_dir: HomeDir<'a>,  /* Home directory.  */
    shell_path: ShellPath<'a>, /* Shell program.  */
}

impl<'a> Passwd<'a> {
    /// Parse a line formatted like one in `/etc/passwd` and construct a matching `Passwd` instance
    ///
    /// # Example
    /// ```
    /// let pwd = adduser::passwd::Passwd::new_from_string(
    ///     "testuser:testpassword:1001:1001:full Name,,,,:/home/test:/bin/test"
    /// ).unwrap();
    /// assert_eq!(pwd.get_username(), "testuser");
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a `UserLibError::Message` containing some information as to why the function failed.
    pub fn new_from_string(line: &'a str) -> Result<Self, UserLibError> {
        let elements: Vec<&str> = line.split(':').collect();
        if elements.len() == 7 {
            Ok(Passwd {
                username: Username::try_from(*elements.get(0).unwrap())?,
                password: Password::try_from(*elements.get(1).unwrap())?,
                uid: Uid::try_from(*elements.get(2).unwrap())?,
                gid: Gid::try_from(*elements.get(3).unwrap())?,
                gecos: Gecos::try_from(*elements.get(4).unwrap())?,
                home_dir: HomeDir::try_from(*elements.get(5).unwrap())?,
                shell_path: ShellPath::try_from(*elements.get(6).unwrap())?,
            })
        } else {
            Err("Failed to parse: not enough elements".into())
        }
    }
    #[must_use]
    pub const fn get_username(&self) -> &'a str {
        self.username.username
    }
    #[must_use]
    pub const fn get_password(&self) -> &'a str {
        self.password.password
    }
    #[must_use]
    pub const fn get_uid(&self) -> u32 {
        self.uid.uid
    }
    #[must_use]
    pub const fn get_gid(&self) -> u32 {
        self.gid.gid
    }
    #[must_use]
    pub const fn get_comment(&self) -> &Gecos {
        &self.gecos
    }
    #[must_use]
    pub const fn get_home_dir(&self) -> &'a str {
        self.home_dir.dir
    }
    #[must_use]
    pub const fn get_shell_path(&self) -> &'a str {
        self.shell_path.shell
    }
}

impl Default for Passwd<'_> {
    fn default() -> Self {
        Passwd {
            username: Username {
                username: "defaultuser",
            },
            password: Password {
                password: "notencrypted",
            },
            uid: Uid { uid: 1001 },
            gid: Gid { gid: 1001 },
            gecos: Gecos::Simple {
                comment: "gecos default comment",
            },
            home_dir: HomeDir {
                dir: "/home/default",
            },
            shell_path: ShellPath { shell: "/bin/bash" },
        }
    }
}

impl Display for Passwd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}",
            self.username,
            self.password,
            self.uid,
            self.gid,
            self.gecos,
            self.home_dir,
            self.shell_path
        )
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uid,)
    }
}

impl TryFrom<&str> for Uid {
    type Error = UserLibError;
    fn try_from(source: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            uid: source.parse::<u32>().unwrap(),
        })
    }
}

impl Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.gid,)
    }
}

impl TryFrom<&str> for Gid {
    type Error = UserLibError;
    fn try_from(source: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            gid: source.parse::<u32>().unwrap(),
        })
    }
}

impl Display for HomeDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dir,)
    }
}

impl<'a> TryFrom<&'a str> for HomeDir<'a> {
    type Error = UserLibError;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Self { dir: source })
    }
}

impl Display for ShellPath<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shell,)
    }
}

impl<'a> TryFrom<&'a str> for ShellPath<'a> {
    type Error = UserLibError;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(ShellPath { shell: source })
    }
}

// Tests ----------------------------------------------------------------------

#[test]
fn test_username_validation() {
    // Failing tests
    let umlauts = Username::try_from("täst"); // umlauts
    assert_eq!(
        Err(UserLibError::Message("Invalid username täst".into())),
        umlauts
    );
    let number_first = Username::try_from("11elf"); // numbers first
    assert_eq!(
        Err(UserLibError::Message("Invalid username 11elf".into())),
        number_first
    );
    let slashes = Username::try_from("test/name"); // slashes in the name
    assert_eq!(
        Err(UserLibError::Message("Invalid username test/name".into())),
        slashes
    );
    let long = Username::try_from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"); // maximum size 32 letters
    assert_eq!(
        Err(UserLibError::Message(
            "Invalid username aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()
        )),
        long
    );
    // Working tests
    let ubuntu_exception = Username::try_from("Debian-exim"); // for some reason ubuntu and debian have a capital user.
    assert_eq!(ubuntu_exception.unwrap().username, "Debian-exim");
    let single = Username::try_from("t"); // single characters are ok
    assert_eq!(single.unwrap().username, "t");
    let normal = Username::try_from("superman"); // regular username
    assert_eq!(normal.unwrap().username, "superman");
    let normal = Username::try_from("anna3pete"); // regular username containing a number
    assert_eq!(normal.unwrap().username, "anna3pete");
    let normal = Username::try_from("enya$"); // regular username ending in a $
    assert_eq!(normal.unwrap().username, "enya$");
}

#[test]
fn test_default_user() {
    // Check if a user can be created.
    let pwd = Passwd::default();
    assert_eq!(pwd.username.username, "defaultuser");
    assert_eq!(pwd.home_dir.dir, "/home/default");
    assert_eq!(pwd.uid.uid, 1001);
}

#[test]
fn test_parse_gecos() {
    // test if the Gecos field can be parsed and the resulting struct is populated correctly.
    let gcdetail = "Full Name,504,11345342,ä1-2312,myemail@test.com";
    let gcsimple = "A böring comment →";
    let gc_no_other: &str = "systemd Network Management,,,";
    let res_detail = Gecos::try_from(gcdetail).unwrap();
    let res_simple = Gecos::try_from(gcsimple).unwrap();
    let res_no_other = Gecos::try_from(gc_no_other).unwrap();
    match res_simple {
        Gecos::Simple { comment } => assert_eq!(comment, "A böring comment →"),
        _ => unreachable!(),
    }
    match res_detail {
        Gecos::Detail {
            full_name,
            room,
            phone_work,
            phone_home,
            other,
        } => {
            assert_eq!(full_name, "Full Name");
            assert_eq!(room, "504");
            assert_eq!(phone_work, "11345342");
            assert_eq!(phone_home, "ä1-2312");
            assert_eq!(other.unwrap()[0], "myemail@test.com");
        }
        _ => unreachable!(),
    }
    match res_no_other {
        Gecos::Detail {
            full_name,
            room,
            phone_work,
            phone_home,
            other,
        } => {
            assert_eq!(full_name, "systemd Network Management");
            assert_eq!(room, "");
            assert_eq!(phone_work, "");
            assert_eq!(phone_home, "");
            assert_eq!(other, None);
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_gecos_getters() {
    // test if the Gecos field can be parsed and the resulting struct is populated correctly.
    let gcdetail = "Full Name,504,11345342,ä1-2312,myemail@test.com";
    let gcsimple = "A böring comment →";
    let gc_no_other: &str = "systemd Network Management,,,";
    let res_detail = Gecos::try_from(gcdetail).unwrap();
    let res_simple = Gecos::try_from(gcsimple).unwrap();
    let res_no_other = Gecos::try_from(gc_no_other).unwrap();
    assert_eq!(res_simple.get_comment(), Some("A böring comment →"));

    assert_eq!(res_detail.get_comment(), None);
    println!("{:?}", res_detail);
    assert_eq!(res_detail.get_full_name(), Some("Full Name"));
    assert_eq!(res_detail.get_room(), Some("504"));
    assert_eq!(res_detail.get_phone_work(), Some("11345342"));
    assert_eq!(res_detail.get_phone_home(), Some("ä1-2312"));
    assert_eq!(res_detail.get_other(), Some(&vec!["myemail@test.com"]));

    assert_eq!(
        res_no_other.get_full_name(),
        Some("systemd Network Management")
    );
    assert_eq!(res_no_other.get_room(), None);
    assert_eq!(res_no_other.get_phone_work(), None);
    assert_eq!(res_no_other.get_phone_home(), None);
    assert_eq!(res_no_other.get_other(), None);
}

#[test]
fn test_new_from_string() {
    // Test if a single line can be parsed and if the resulting struct is populated correctly.
    let fail = Passwd::new_from_string("").err().unwrap();
    assert_eq!(
        fail,
        UserLibError::Message("Failed to parse: not enough elements".into())
    );
    let pwd =
        Passwd::new_from_string("testuser:testpassword:1001:1001:testcomment:/home/test:/bin/test")
            .unwrap();
    let pwd2 =
        Passwd::new_from_string("testuser:testpassword:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test")
            .unwrap();
    assert_eq!(pwd.username.username, "testuser");
    assert_eq!(pwd.home_dir.dir, "/home/test");
    assert_eq!(pwd.uid.uid, 1001);
    match pwd.gecos {
        Gecos::Simple { comment } => assert_eq!(comment, "testcomment"),
        _ => unreachable!(),
    }
    match pwd2.gecos {
        Gecos::Detail {
            full_name,
            room,
            phone_work,
            phone_home,
            other,
        } => {
            assert_eq!(full_name, "full Name");
            assert_eq!(room, "004");
            assert_eq!(phone_work, "000342");
            assert_eq!(phone_home, "001-2312");
            assert_eq!(other.unwrap()[0], "myemail@test.com");
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_parse_passwd() {
    // Test wether the passwd file can be parsed and recreated without throwing an exception
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let file = File::open("/etc/passwd").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let lineorig: String = line.unwrap();
        let linecopy = lineorig.clone();
        let pass_struc = Passwd::new_from_string(&linecopy).unwrap();
        assert_eq!(
            // ignoring the numbers of `,` since the implementation does not (yet) reproduce a missing comment field.
            lineorig,
            format!("{}", pass_struc)
        );
    }
}
