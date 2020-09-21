use std::cmp::Eq;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Username<'a> {
    pw_name: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Password<'a> {
    pw_passwd: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Uid {
    pw_uid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gid {
    pw_gid: u32,
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
    pw_dir: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ShellDir<'a> {
    pw_shell: &'a str,
}

/// A record in the user database `/etc/passwd`.
#[derive(Debug, PartialEq, Eq)]
pub struct Passwd<'a> {
    pw_name: Username<'a>,   /* Username.  */
    pw_passwd: Password<'a>, /* Hashed passphrase, if shadow database not in use (see shadow.h).  */
    pw_uid: Uid,             /* User ID.  */
    pw_gid: Gid,             /* Group ID.  */
    pw_gecos: Gecos<'a>,     /* Real name.  */
    pw_dir: HomeDir<'a>,     /* Home directory.  */
    pw_shell: ShellDir<'a>,  /* Shell program.  */
}

impl<'a> Passwd<'a> {
    pub fn new_from_string(line: &'a str) -> Result<Self, &str> {
        let elements: Vec<&str> = line.split(":").collect();
        if elements.len() != 7 {
            return Err("Failed to parse: not enough elements");
        } else {
            Ok(Passwd {
                pw_name: Username {
                    pw_name: elements.get(0).unwrap(),
                },
                pw_passwd: Password {
                    pw_passwd: elements.get(1).unwrap(),
                },
                pw_uid: Uid {
                    pw_uid: elements.get(2).unwrap().parse::<u32>().unwrap(),
                },
                pw_gid: Gid {
                    pw_gid: elements.get(3).unwrap().parse::<u32>().unwrap(),
                },
                pw_gecos: parse_gecos(elements.get(4).unwrap()).unwrap(),
                pw_dir: HomeDir {
                    pw_dir: elements.get(5).unwrap(),
                },
                pw_shell: ShellDir {
                    pw_shell: elements.get(6).unwrap(),
                },
            })
        }
    }
}

fn parse_gecos(source: &str) -> Result<Gecos, &str> {
    let vals: Vec<&str> = source.split(',').collect();
    if vals.len() == 5 {
        Ok(Gecos::Detail {
            full_name: vals.get(0).unwrap(),
            room: vals.get(1).unwrap(),
            phone_work: vals.get(2).unwrap(),
            phone_home: vals.get(3).unwrap(),
            other: vals.get(4).unwrap(),
        })
    } else if vals.len() == 1 {
        Ok(Gecos::Simple {
            comment: vals.get(0).unwrap(),
        })
    } else {
        panic!(format!("Could not parse this string: {}", source))
    }
}

impl Default for Passwd<'_> {
    fn default() -> Self {
        Passwd {
            pw_name: Username { pw_name: "howdy" },
            pw_passwd: Password {
                pw_passwd: "notencrypted",
            },
            pw_uid: Uid { pw_uid: 1001 },
            pw_gid: Gid { pw_gid: 1001 },
            pw_gecos: Gecos::Simple {
                comment: "not done",
            },
            pw_dir: HomeDir {
                pw_dir: "/home/test",
            },
            pw_shell: ShellDir {
                pw_shell: "/bin/bash",
            },
        }
    }
}

impl Display for Passwd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}",
            self.pw_name,
            self.pw_passwd,
            self.pw_uid,
            self.pw_gid,
            self.pw_gecos,
            self.pw_dir,
            self.pw_shell
        )
    }
}

impl Display for Username<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_name,)
    }
}

impl Display for Password<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_passwd,)
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_uid,)
    }
}

impl Display for Gid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_gid,)
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

impl Display for HomeDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_dir,)
    }
}

impl Display for ShellDir<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pw_shell,)
    }
}
