pub mod gecos_fields;
pub mod passwd_fields;
pub mod shadow_fields;

use crate::userlib::NewFromString;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// A record(line) in the user database `/etc/passwd` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct User {
    source: String,
    username: crate::Username,            /* Username.  */
    pub(crate) password: crate::Password, /* Hashed passphrase, if shadow database not in use (see shadow.h).  */
    uid: crate::Uid,                      /* User ID.  */
    gid: crate::Gid,                      /* Group ID.  */
    gecos: crate::Gecos,                  /* Real name.  */
    home_dir: crate::HomeDir,             /* Home directory.  */
    shell_path: crate::ShellPath,         /* Shell program.  */
}

impl NewFromString for User {
    /// Parse a line formatted like one in `/etc/passwd` and construct a matching [`User`] instance
    ///
    /// # Example
    /// ```
    /// use crate::adduser::api::UserRead;
    /// use adduser::NewFromString;
    /// let pwd = adduser::User::new_from_string(
    ///     "testuser:testpassword:1001:1001:full Name,,,,:/home/test:/bin/test".to_string()).unwrap();
    /// assert_eq!(pwd.get_username().unwrap(), "testuser");
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a [`UserLibError::Message`](crate::userlib_error::UserLibError::Message) containing some information as to why the function failed.
    fn new_from_string(line: String) -> Result<Self, crate::UserLibError>
    where
        Self: Sized,
    {
        let elements: Vec<String> = line.split(':').map(ToString::to_string).collect();
        if elements.len() == 7 {
            Ok(Self {
                source: line,
                username: crate::Username::try_from(elements.get(0).unwrap().to_string())?,
                password: crate::Password::Encrypted(crate::EncryptedPassword::try_from(
                    elements.get(1).unwrap().to_string(),
                )?),
                uid: crate::Uid::try_from(elements.get(2).unwrap().to_string())?,
                gid: crate::Gid::try_from(elements.get(3).unwrap().to_string())?,
                gecos: crate::Gecos::try_from(elements.get(4).unwrap().to_string())?,
                home_dir: crate::HomeDir::try_from(elements.get(5).unwrap().to_string())?,
                shell_path: crate::ShellPath::try_from(elements.get(6).unwrap().to_string())?,
            })
        } else {
            Err("Failed to parse: not enough elements".into())
        }
    }
}

impl crate::api::UserRead for User {
    #[must_use]
    fn get_username(&self) -> Option<&str> {
        Some(&self.username.username)
    }
    #[must_use]
    fn get_password(&self) -> Option<&str> {
        match &self.password {
            crate::Password::Encrypted(crate::EncryptedPassword { password }) => Some(&password),
            crate::Password::Shadow(crate::Shadow { ref password, .. }) => Some(&password.password),
            crate::Password::Disabled => None,
        }
    }
    #[must_use]
    fn get_uid(&self) -> u32 {
        self.uid.uid
    }
    #[must_use]
    fn get_gid(&self) -> u32 {
        self.gid.gid
    }
    #[must_use]
    fn get_gecos(&self) -> Option<&crate::Gecos> {
        Some(&self.gecos)
    }
    #[must_use]
    fn get_home_dir(&self) -> Option<&str> {
        Some(&self.home_dir.dir)
    }
    #[must_use]
    fn get_shell_path(&self) -> Option<&str> {
        Some(&self.shell_path.shell)
    }

    fn get_full_name(&self) -> Option<&str> {
        self.gecos.get_full_name()
    }

    fn get_room(&self) -> Option<&str> {
        self.gecos.get_room()
    }

    fn get_phone_work(&self) -> Option<&str> {
        self.gecos.get_phone_work()
    }

    fn get_phone_home(&self) -> Option<&str> {
        self.gecos.get_phone_home()
    }

    fn get_other(&self) -> Option<&Vec<String>> {
        self.gecos.get_other()
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            source: "".to_owned(),
            username: crate::Username {
                username: "defaultuser".to_owned(),
            },
            password: crate::Password::Encrypted(crate::EncryptedPassword {
                password: "notencrypted".to_owned(),
            }),
            uid: crate::Uid { uid: 1001 },
            gid: crate::Gid { gid: 1001 },
            gecos: crate::Gecos::Simple {
                comment: "gecos default comment".to_string(),
            },
            home_dir: crate::HomeDir {
                dir: "/home/default".to_owned(),
            },
            shell_path: crate::ShellPath {
                shell: "/bin/bash".to_owned(),
            },
        }
    }
}

impl Display for User {
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

#[test]
fn test_default_user() {
    // Check if a user can be created.
    let pwd = User::default();
    assert_eq!(pwd.username.username, "defaultuser");
    assert_eq!(pwd.home_dir.dir, "/home/default");
    assert_eq!(pwd.uid.uid, 1001);
}

#[test]
fn test_new_from_string() {
    // Test if a single line can be parsed and if the resulting struct is populated correctly.
    let fail = User::new_from_string("".into()).err().unwrap();
    assert_eq!(
        fail,
        crate::UserLibError::Message("Failed to parse: not enough elements".into())
    );
    let pwd = User::new_from_string(
        "testuser:testpassword:1001:1001:testcomment:/home/test:/bin/test".into(),
    )
    .unwrap();
    let pwd2 =
        User::new_from_string("testuser:testpassword:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test".into())
            .unwrap();
    assert_eq!(pwd.username.username, "testuser");
    assert_eq!(pwd.home_dir.dir, "/home/test");
    assert_eq!(pwd.uid.uid, 1001);
    match pwd.gecos {
        crate::Gecos::Simple { comment } => assert_eq!(comment, "testcomment"),
        _ => unreachable!(),
    }
    match pwd2.gecos {
        crate::Gecos::Detail {
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
        let pass_struc = User::new_from_string(linecopy).unwrap();
        assert_eq!(
            // ignoring the numbers of `,` since the implementation does not (yet) reproduce a missing comment field.
            lineorig,
            format!("{}", pass_struc)
        );
    }
}
