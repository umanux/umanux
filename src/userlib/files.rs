use std::{io::Seek, io::SeekFrom, path::PathBuf};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
use std::ops::Deref;

#[derive(Debug)]
pub struct Files {
    pub passwd: Option<PathBuf>,
    pub shadow: Option<PathBuf>,
    pub group: Option<PathBuf>,
}

impl Default for Files {
    /// use the default Linux `/etc/` paths
    fn default() -> Self {
        Self {
            passwd: Some(PathBuf::from("/etc/passwd")),
            shadow: Some(PathBuf::from("/etc/shadow")),
            group: Some(PathBuf::from("/etc/group")),
        }
    }
}

impl Files {
    /// Check if all the files are defined. Because some operations require the files to be present
    #[must_use]
    pub const fn is_virtual(&self) -> bool {
        !(self.group.is_some() & self.passwd.is_some() & self.shadow.is_some())
    }
    pub fn lock_and_get_passwd(&self) -> Result<LockedFileGuard, crate::UserLibError> {
        let path = self.passwd.as_ref();
        match path {
            Some(p) => LockedFileGuard::new(p),
            None => Err(crate::UserLibError::FilesRequired),
        }
    }
    pub fn lock_and_get_shadow(&self) -> Result<LockedFileGuard, crate::UserLibError> {
        let path = self.shadow.as_ref();
        match path {
            Some(p) => LockedFileGuard::new(p),
            None => Err(crate::UserLibError::FilesRequired),
        }
    }
    pub fn lock_and_get_group(&self) -> Result<LockedFileGuard, crate::UserLibError> {
        let path = self.group.as_ref();
        match path {
            Some(p) => LockedFileGuard::new(p),
            None => Err(crate::UserLibError::FilesRequired),
        }
    }

    pub fn lock_all_get(
        &self,
    ) -> Result<(LockedFileGuard, LockedFileGuard, LockedFileGuard), crate::UserLibError> {
        let pwd = self.lock_and_get_passwd()?;
        let shd = self.lock_and_get_shadow()?;
        let grp = self.lock_and_get_group()?;
        Ok((pwd, shd, grp))
    }
}

#[derive(Debug)]
pub struct LockedFileGuard {
    lockfile: PathBuf,
    path: PathBuf,
    pub(crate) file: File,
}

#[derive(Debug)]
struct TempLockFile {
    tlf: PathBuf,
}

impl Drop for TempLockFile {
    fn drop(&mut self) {
        info!("removing temporary lockfile {}", self.tlf.to_str().unwrap());
        std::fs::remove_file(&self.tlf).unwrap();
    }
}
impl Deref for TempLockFile {
    type Target = PathBuf;
    fn deref(&self) -> &PathBuf {
        &self.tlf
    }
}
impl LockedFileGuard {
    pub fn new(path: &PathBuf) -> Result<Self, crate::UserLibError> {
        let locked = Self::try_to_lock_file(path);
        match locked {
            Ok((lockfile, file)) => Ok(Self {
                lockfile,
                path: path.to_owned(),
                file,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn replace_contents(&mut self, new_content: String) -> Result<(), crate::UserLibError> {
        self.file = match File::create(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(("Failed to truncate file.".to_owned(), e).into()),
        };
        match self.file.write_all(&new_content.into_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(("Could not write (all) users. ".to_owned(), e).into()),
        };
        let _ = self.file.write(b"\n");
        Ok(())
    }

    pub fn append(&mut self, appendee: String) -> Result<(), crate::UserLibError> {
        // Seek to the last character.
        self.file.seek(SeekFrom::End(-1)).map_or_else(
            |e| Err(format!("Failed to append to file {}", e)),
            |_| Ok(()),
        )?;
        // Read the last character
        let mut b = [0 as u8; 1];
        self.file.read_exact(&mut b)?;
        // Verify it is '\n' else append '\n' so in any case the file ends with with a newline now
        if &b != b"\n" {
            //self.file.write_all(&b)?;
            self.file.write_all(b"\n")?;
        }
        // write the new line.
        self.file.write_all(&appendee.into_bytes()).map_or_else(
            |e| Err(("Failed to append to file".to_owned(), e).into()),
            Ok,
        )
    }

    /// This function tries to lock a file in the way other passwd locking mechanisms work.
    ///
    /// * get the pid
    /// * create the temporary lockfilepath "/etc/passwd.12397"
    /// * create the lockfilepath "/etc/passwd.lock"
    /// * open the temporary file
    /// * write the pid to the tempfile
    /// * try to make a link from the temporary file created to the lockfile
    /// * ensure that the file has been linked successfully
    ///
    /// when the link could not be created:
    ///
    /// * Open the lockfile
    /// * read the contents of the lockfile
    /// * check if the lockfile contains a pid if not error out
    /// * check if the containing pid is in a valid format. If not create a matching error
    ///
    /// not implemented yet:
    ///
    /// * test if this process could be killed. If so disclose the pid in the error.
    /// * try to delete the lockfile as it is apparently not used by the process anmore. (cleanup)
    /// * try to lock again now that the old logfile has been safely removed.
    /// * remove the original file and only keep the lock hardlink
    fn try_to_lock_file(path: &PathBuf) -> Result<(PathBuf, File), crate::UserLibError> {
        info!("locking file {}", path.to_string_lossy());
        let mut tempfilepath_const = path.clone();
        // get the pid
        let pid = std::process::id();
        debug!("using pid {}", std::process::id());
        // get the filename
        let filename = tempfilepath_const.file_name().unwrap().to_owned();
        // and the base path which is the base for tempfile and lockfile.
        tempfilepath_const.pop();
        let mut lockfilepath = tempfilepath_const.clone();
        // push the filenames to the paths
        tempfilepath_const.push(format!("{}.{}", filename.to_str().unwrap(), pid));
        let tempfilepath = TempLockFile {
            tlf: tempfilepath_const,
        };
        lockfilepath.push(format!("{}.lock", filename.to_str().unwrap()));
        debug!(
            "Lockfile paths: {:?} (temporary) {:?} (final)",
            *tempfilepath, lockfilepath
        );
        // write the pid into the tempfile
        {
            let mut tempfile = File::create(&*tempfilepath).unwrap_or_else(|e| {
                panic!("Failed to open {} error: {}", filename.to_str().unwrap(), e)
            });
            write!(tempfile, "{}", pid).or_else(|e| {
                let error_msg = format!(
                    "could not write to {} error {}",
                    filename.to_string_lossy(),
                    e
                );
                error!("{}", error_msg);
                let err: crate::UserLibError = error_msg.into();
                Err(err)
            })?;
        }

        // try to make a hardlink from the lockfile to the tempfile
        let linkresult = std::fs::hard_link(&*tempfilepath, &lockfilepath);
        match linkresult {
            Ok(()) => {
                debug!("successfully locked");

                // open the file
                let resfile = OpenOptions::new().read(true).write(true).open(&path);
                return match resfile {
                    Ok(file) => Ok((lockfilepath, file)),
                    Err(e) => {
                        // failed to open the file undo the locks
                        let _ = std::fs::remove_file(&lockfilepath);
                        let ret: crate::UserLibError = format!(
                            "Failed to open the file: {}, error: {}",
                            path.to_str().unwrap(),
                            e
                        )
                        .into();
                        Err(ret)
                    }
                };
            }
            Err(e) => match e.kind() {
                // analyze the error further
                std::io::ErrorKind::AlreadyExists => {
                    warn!("The file is already locked by another process! – testing the validity of the lock");
                    {
                        let mut lf = match File::open(&lockfilepath) {
                            Ok(file) => file,
                            Err(e) => {
                                panic!("failed to open the lockfile: {}", e);
                            }
                        };
                        let mut content = String::new();
                        lf.read_to_string(&mut content)
                            .unwrap_or_else(|e| panic!("failed to read the lockfile{}", e));

                        let content = content.trim().trim_matches(char::from(0));
                        let lock_pid = content.parse::<u32>();
                        match lock_pid {
                            Ok(pid) => {
                                warn!(
                                    "found a pid: {}, checking if this process is still running",
                                    pid
                                );
                                error!("The file could not be locked");
                                todo!("Validate the lock and delete the file if the process does not exist anymore");
                                /*let sent = nix::sys::signal::kill(
                                    nix::unistd::Pid::from_raw(pid as i32),
                                    nix::sys::signal::Signal::from(0),
                                );*/
                            }
                            Err(e) => error!(
                                "existing lock file {} with an invalid PID '{}' Error: {}",
                                lockfilepath.to_str().unwrap(),
                                content,
                                e
                            ),
                        }
                    }
                }

                _ => {
                    panic!("failed to lock the file: {}", e);
                }
            },
        }
        Err("was not able to lock!".into())
    }
}

impl Drop for LockedFileGuard {
    fn drop(&mut self) {
        info!("removing lock");
        std::fs::remove_file(&self.lockfile).unwrap();
    }
}
