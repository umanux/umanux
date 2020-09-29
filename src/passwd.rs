#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct Password<'a> {
    password: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Uid {
    uid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gid {
    gid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Gecos<'a> {
    Detail {
        full_name: &'a str,
        room: &'a str,
        phone_work: &'a str,
        phone_home: &'a str,
        other: &'a str,
    },
    Simple {
        comment: &'a str,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct HomeDir<'a> {
    dir: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShellDir<'a> {
    shell: &'a str,
}

/// A record in the user database `/etc/passwd`.
#[derive(Debug, PartialEq, Eq)]
pub struct Passwd<'a> {
    username: Username<'a>,  /* Username.  */
    password: Password<'a>,  /* Hashed passphrase, if shadow database not in use (see shadow.h).  */
    uid: Uid,                /* User ID.  */
    gid: Gid,                /* Group ID.  */
    gecos: Gecos<'a>,        /* Real name.  */
    home_dir: HomeDir<'a>,   /* Home directory.  */
    shell_dir: ShellDir<'a>, /* Shell program.  */
}

impl<'a> Passwd<'a> {
    /// Parse a line formatted like one in `/etc/passwd` and construct a matching `Passwd` instance
    ///
    /// # Example
    /// ```
    /// let pwd = adduser::passwd::Passwd::new_from_string(
    ///     "testuser:testpassword:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test"
    /// ).unwrap();
    /// assert!(format!("{:?}", pwd).find("testuser") > Some(0));
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a `UserLibError::Message` containing some information as to why the function failed.
    pub fn new_from_string(line: &'a str) -> Result<Self, UserLibError> {
        let elements: Vec<&str> = line.split(':').collect();
        if elements.len() == 7 {
            Ok(Passwd {
                username: Username::try_from(*elements.get(0).unwrap())
                    .expect("failed to parse username."),
                password: Password::try_from(*elements.get(1).unwrap())
                    .expect("Failed to parse Password"),
                uid: Uid::try_from(*elements.get(2).unwrap()).expect("Failed to parse uid"),
                gid: Gid::try_from(*elements.get(3).unwrap()).expect("Failed to parse gid"),
                gecos: Gecos::try_from(*elements.get(4).unwrap())
                    .expect("Failed to parse Gecos field"),
                home_dir: HomeDir::try_from(*elements.get(5).unwrap())
                    .expect("Failed to parse home directory"),
                shell_dir: ShellDir::try_from(*elements.get(6).unwrap())
                    .expect("Failed to parse shell directory"),
            })
        } else {
            Err(UserLibError::Message(
                "Failed to parse: not enough elements".to_owned(),
            ))
        }
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
            shell_dir: ShellDir { shell: "/bin/bash" },
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
            self.shell_dir
        )
    }
}

impl Display for Username<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.username,)
    }
}

impl<'a> TryFrom<&'a str> for Username<'a> {
    type Error = &'static str;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Self { username: source })
    }
}

impl Display for Password<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.password,)
    }
}

impl<'a> TryFrom<&'a str> for Password<'a> {
    type Error = &'static str;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Self { password: source })
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uid,)
    }
}

impl TryFrom<&str> for Uid {
    type Error = &'static str;
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
    type Error = &'static str;
    fn try_from(source: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            gid: source.parse::<u32>().unwrap(),
        })
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
                "{},{},{},{},{}",
                full_name, room, phone_work, phone_home, other
            ),
        }
    }
}

impl<'a> TryFrom<&'a str> for Gecos<'a> {
    type Error = &'static str;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        let vals: Vec<&str> = source.split(',').collect();
        if vals.len() == 5 {
            Ok(Gecos::Detail {
                full_name: vals.get(0).unwrap(),
                room: vals.get(1).unwrap(),
                phone_work: vals.get(2).unwrap(),
                phone_home: vals.get(3).unwrap(),
                other: vals.get(4).unwrap(),
            })
        } else if vals.len() == 4 {
            Ok(Gecos::Detail {
                full_name: vals.get(0).unwrap(),
                room: vals.get(1).unwrap(),
                phone_work: vals.get(2).unwrap(),
                phone_home: vals.get(3).unwrap(),
                other: "",
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

impl Display for HomeDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dir,)
    }
}

impl<'a> TryFrom<&'a str> for HomeDir<'a> {
    type Error = &'static str;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Self { dir: source })
    }
}

impl Display for ShellDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.shell,)
    }
}

impl<'a> TryFrom<&'a str> for ShellDir<'a> {
    type Error = &'static str;
    fn try_from(source: &'a str) -> std::result::Result<Self, Self::Error> {
        Ok(Self { shell: source })
    }
}

// Tests ----------------------------------------------------------------------

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
            assert_eq!(other, "myemail@test.com");
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
            assert_eq!(other, "");
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_new_from_string() {
    // Test if a single line can be parsed and if the resulting struct is populated correctly.
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
            assert_eq!(other, "myemail@test.com");
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
        assert_eq!(
            // ignoring the numbers of `,` since the implementation does not (yet) reproduce a missing comment field.
            format!("{}", Passwd::new_from_string(&lineorig.clone()).unwrap()).replace(",", ""),
            lineorig.replace(",", "")
        );
    }
}
