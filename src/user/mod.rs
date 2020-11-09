pub mod gecos_fields;

pub mod passwd_fields;
pub mod shadow_fields;

use crate::userlib::NewFromString;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// A record(line) in the user database `/etc/passwd` found in most linux systems.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    source: String,
    pos: u32,
    username: crate::Username,            /* Username.  */
    pub(crate) password: crate::Password, /* Hashed passphrase, if shadow database not in use (see shadow.h).  */
    uid: crate::Uid,                      /* User ID.  */
    gid: crate::Gid,                      /* Group ID.  */
    gecos: crate::Gecos,                  /* Real name.  */
    home_dir: crate::HomeDir,             /* Home directory.  */
    shell_path: crate::ShellPath,         /* Shell program.  */
}

impl User {
    #[must_use]
    pub const fn get_shadow(&self) -> Option<&crate::Shadow> {
        match self.password {
            crate::Password::Encrypted(_) | crate::Password::Disabled => None,
            crate::Password::Shadow(ref s) => Some(s),
        }
    }
    /*fn get_nth_line(content: &str, n: u32) -> (String, u64) {
        use std::io::BufRead;
        let mut cursor = std::io::Cursor::new(content);

        let mut offset: u64 = 0;
        let mut tmp = String::new();
        for _ in 0..n + 1 {
            tmp = String::new();
            offset += cursor.read_line(&mut tmp).expect("failed to read line") as u64;
            //trace!("reading user{}: {}", i, &tmp);
        }
        (tmp.trim().to_owned(), offset)
    }
    pub(crate) fn is_unchanged_in(&self, content: &str) -> (u64, usize, bool) {
        let (line, offset) = Self::get_nth_line(content, self.pos);
        trace!("Olduser:\n\t{}\nNewuser\n\t{}", &self.source, &line);
        (offset, line.len(), line == self.source)
    }*/
    #[must_use]
    pub fn remove_in(&self, content: &str) -> String {
        content
            .split(&self.source)
            .map(str::trim)
            .collect::<Vec<&str>>()
            .join("\n")
    }
    pub fn username(&mut self, name: String) -> &mut Self {
        self.username = crate::Username { username: name };
        self
    }
    pub fn disable_password(&mut self) -> &mut Self {
        self.password = crate::Password::Disabled;
        self
    }
    pub fn uid(&mut self, uid: u32) -> &mut Self {
        self.uid = crate::Uid { uid };
        self
    }
    pub fn gid(&mut self, gid: u32) -> &mut Self {
        self.gid = crate::Gid { gid };
        self
    }
    pub fn home_dir(&mut self, path: String) -> &mut Self {
        self.home_dir = crate::HomeDir { dir: path };
        self
    }
    pub fn shell_path(&mut self, path: String) -> &mut Self {
        self.shell_path = crate::ShellPath { shell: path };
        self
    }
}

impl NewFromString for User {
    /// Parse a line formatted like one in `/etc/passwd` and construct a matching [`User`] instance
    ///
    /// # Example
    /// ```
    /// use crate::adduser::api::UserRead;
    /// use adduser::NewFromString;
    /// let pwd = adduser::User::new_from_string(
    ///     "testuser:testpassword:1001:1001:full Name,,,,:/home/test:/bin/test".to_string(), 0).unwrap();
    /// assert_eq!(pwd.get_username().unwrap(), "testuser");
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a [`UserLibError::Message`](crate::userlib_error::UserLibError::Message) containing some information as to why the function failed.
    fn new_from_string(line: String, position: u32) -> Result<Self, crate::UserLibError>
    where
        Self: Sized,
    {
        let elements: Vec<String> = line.split(':').map(ToString::to_string).collect();
        if elements.len() == 7 {
            Ok(Self {
                source: line,
                pos: position,
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
            crate::Password::Encrypted(crate::EncryptedPassword { password }) => Some(password),
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

    #[must_use]
    fn get_full_name(&self) -> Option<&str> {
        self.gecos.get_full_name()
    }

    #[must_use]
    fn get_room(&self) -> Option<&str> {
        self.gecos.get_room()
    }

    #[must_use]
    fn get_phone_work(&self) -> Option<&str> {
        self.gecos.get_phone_work()
    }

    #[must_use]
    fn get_phone_home(&self) -> Option<&str> {
        self.gecos.get_phone_home()
    }

    #[must_use]
    fn get_other(&self) -> Option<&Vec<String>> {
        self.gecos.get_other()
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.pos.cmp(&other.pos))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            source: "".to_owned(),
            pos: u32::MAX,
            username: crate::Username {
                username: "defaultusername".to_owned(),
            },
            password: crate::Password::Disabled,
            uid: crate::Uid { uid: 1001 },
            gid: crate::Gid { gid: 1001 },
            gecos: crate::Gecos::Simple {
                comment: "".to_string(),
            },
            home_dir: crate::HomeDir {
                dir: "/".to_owned(),
            },
            shell_path: crate::ShellPath {
                shell: "/bin/nologin".to_owned(),
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
    let mut pwd = User::default();
    assert_eq!(pwd.username.username, "defaultusername");
    assert_eq!(pwd.home_dir.dir, "/");
    assert_eq!(pwd.uid.uid, 1001);
    let npw = pwd.username("test".to_owned()).clone();
    assert_eq!(npw.username.username, "test");
}

#[test]
fn test_new_from_string() {
    // Test if a single line can be parsed and if the resulting struct is populated correctly.
    let fail = User::new_from_string("".into(), 0).err().unwrap();
    assert_eq!(fail, "Failed to parse: not enough elements".into());
    let pwd = User::new_from_string(
        "testuser:testpassword:1001:1001:testcomment:/home/test:/bin/test".into(),
        0,
    )
    .unwrap();
    let pwd2 =
        User::new_from_string("testuser:testpassword:1001:1001:full Name,004,000342,001-2312,myemail@test.com:/home/test:/bin/test".into(),0)
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
    use std::convert::TryInto;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    let file = File::open("/etc/passwd").unwrap();
    let reader = BufReader::new(file);

    for (n, line) in reader.lines().enumerate() {
        let lineorig: String = line.unwrap();
        let linecopy = lineorig.clone();
        let pass_struc = User::new_from_string(linecopy, n.try_into().unwrap()).unwrap();
        assert_eq!(
            // ignoring the numbers of `,` since the implementation does not (yet) reproduce a missing comment field.
            lineorig,
            format!("{}", pass_struc)
        );
    }
}
