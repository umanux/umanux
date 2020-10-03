#![warn(
    clippy::all,
/*    clippy::restriction,*/
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::non_ascii_literal)]

use log::warn;

use crate::passwd;
use crate::userlib_error::UserLibError;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Display};

/// A record(line) in the user database `/etc/shadow` found in most linux systems.
#[derive(Debug, PartialEq, Eq)]
pub struct Shadow<'a> {
    username: passwd::Username<'a>,                 /* Username.  */
    password: passwd::Password<'a>,                 /* Hashed passphrase */
    last_change: Option<chrono::NaiveDateTime>,     /* User ID.  */
    earliest_change: Option<chrono::NaiveDateTime>, /* Group ID.  */
    latest_change: Option<chrono::NaiveDateTime>,   /* Real name.  */
    warn_period: Option<chrono::Duration>,          /* Home directory.  */
    deactivated: Option<chrono::Duration>,          /* Shell program.  */
    deactivated_since: Option<chrono::Duration>,    /* Shell program.  */
    extensions: Option<u64>,                        /* Shell program.  */
}

impl<'a> Display for Shadow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.username,
            self.password,
            show_option_date(self.last_change),
            show_option_date(self.earliest_change),
            show_option_date(self.latest_change),
            show_option_duration(self.warn_period),
            show_option_duration(self.deactivated),
            show_option_duration(self.deactivated_since),
            if self.extensions.is_none() {
                "".to_string()
            } else {
                self.extensions.unwrap().to_string()
            }
        )
    }
}

fn show_option_date(input: Option<chrono::NaiveDateTime>) -> String {
    if input.is_none() {
        "".into()
    } else {
        format!("{}", input.unwrap().timestamp() / SECONDS_PER_DAY)
    }
}

fn show_option_duration(input: Option<chrono::Duration>) -> String {
    if input.is_none() {
        "".into()
    } else {
        format!("{}", input.unwrap().num_days())
    }
}

impl<'a> Shadow<'a> {
    /// Parse a line formatted like one in `/etc/shadow` and construct a matching `Shadow` instance
    ///
    /// # Example
    /// ```
    /// let pwd = adduser::shadow::Shadow::new_from_string(
    ///     "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::"
    /// ).unwrap();
    ///
    /// ```
    ///
    /// # Errors
    /// When parsing fails this function returns a `UserLibError::Message` containing some information as to why the function failed.
    pub fn new_from_string(line: &'a str) -> Result<Self, UserLibError> {
        println!("{}", &line);
        let elements: Vec<&str> = line.split(':').collect();
        if elements.len() == 9 {
            let extra = elements.get(8).unwrap();
            Ok(Shadow {
                username: passwd::Username::try_from(*elements.get(0).unwrap())?,
                password: passwd::Password::try_from(*elements.get(1).unwrap())?,
                last_change: date_since_epoch(elements.get(2).unwrap()),
                earliest_change: date_since_epoch(elements.get(3).unwrap()),
                latest_change: date_since_epoch(elements.get(4).unwrap()),
                warn_period: duration_for_days(elements.get(5).unwrap()),
                deactivated: duration_for_days(elements.get(6).unwrap()),
                deactivated_since: duration_for_days(elements.get(7).unwrap()),
                extensions: if extra.is_empty() {
                    None
                } else {
                    Some(extra.parse::<u64>().unwrap())
                },
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
    println!("Test");
    let line = "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::";
    let line2 = Shadow::new_from_string(line).unwrap();
    println!("{:#?}", line2);
    assert_eq!(format!("{}", line2), line);
}
