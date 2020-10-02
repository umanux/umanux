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

use crate::passwd;
use crate::userlib_error::UserLibError;
use chrono;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// A record(line) in the user database `/etc/shadow` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct Shadow<'a> {
    username: passwd::Username<'a>,                 /* Username.  */
    password: passwd::Password<'a>,                 /* Hashed passphrase */
    last_change: Option<chrono::NaiveDateTime>,     /* User ID.  */
    earliest_change: Option<chrono::NaiveDateTime>, /* Group ID.  */
    lateste_change: Option<chrono::NaiveDateTime>,  /* Real name.  */
    warn_period: Option<chrono::Duration>,          /* Home directory.  */
    deactivated: Option<chrono::Duration>,          /* Shell program.  */
    deactivated_since: Option<chrono::Duration>,    /* Shell program.  */
    extensions: Option<u64>,                        /* Shell program.  */
}

impl<'a> Shadow<'a> {
    /// Parse a line formatted like one in `/etc/shadow` and construct a matching `Shadow` instance
    ///
    /// # Example
    /// ```
    /// let pwd = adduser::shadow::Shadow::new_from_string(
    ///     ""
    /// ).unwrap();
    /// //assert_eq!(pwd.get_username(), "testuser");
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a `UserLibError::Message` containing some information as to why the function failed.
    pub fn new_from_string(line: &'a str) -> Result<Self, UserLibError> {
        let elements: Vec<&str> = line.split(':').collect();
        if elements.len() == 9 {
            Ok(Shadow {
                username: passwd::Username::try_from(*elements.get(0).unwrap())?,
                password: passwd::Password::try_from(*elements.get(1).unwrap())?,
                last_change: date_since_epoch(elements.get(2).unwrap()),
                earliest_change: date_since_epoch(elements.get(3).unwrap()),
                lateste_change: date_since_epoch(elements.get(4).unwrap()),
                warn_period: duration_for_days(elements.get(5).unwrap()),
                deactivated: duration_for_days(elements.get(6).unwrap()),
                deactivated_since: duration_for_days(elements.get(7).unwrap()),
                extensions: Some(0),
            })
        } else {
            Err("Failed to parse: not enough elements".into())
        }
    }
}

const SECONDS_PER_DAY: i64 = 86400;
fn date_since_epoch(days_since_epoch: &str) -> Option<chrono::NaiveDateTime> {
    if days_since_epoch.is_empty() {
        None
    } else {
        let days: i64 = days_since_epoch.parse::<i64>().unwrap();
        let seconds = days * SECONDS_PER_DAY;
        Some(chrono::NaiveDateTime::from_timestamp(seconds, 0))
    }
}
fn duration_for_days(days_source: &str) -> Option<chrono::Duration> {
    if days_source.is_empty() {
        None
    } else {
        let days: i64 = days_source.parse::<i64>().unwrap();
        Some(chrono::Duration::days(days))
    }
}

#[test]
fn test_since_epoch() {
    panic!(format!(
        "{:?}",
        Shadow::new_from_string("dietrich:$6$SCJjPV7$SZ7XgOdEMiqZ3v5n9Q2AR2yJKN0PLbSHlrdiZcp/NcB41JEtT12Ke3Zy6XThfiFemJheC0IrM3..JVCAagqxg.:18110:0:99999:7:::")
    ));
}
