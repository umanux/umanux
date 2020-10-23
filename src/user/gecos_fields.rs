use crate::UserLibError;
use std::cmp::Eq;
use std::convert::TryFrom;
use std::fmt::{self, Display};
/// The gecos field of a user.
///
/// In the `/etc/passwd` file this field is a `,` sepparated list of items.
/// The first 4 values are more or less standardised to be full name, room, phone at work and phone at home. After that there can be some extra fields often containing the emailadress and even additional information.
///
/// This enum represents the first 4 values by name and adds the other values to a list of strings [`Gecos::Detail`]. If only one field is found and no `,` at all this value is used as a human readable comment [`Gecos::Simple`].
#[derive(Debug, PartialEq, Eq)]
pub enum Gecos {
    Detail {
        full_name: String,
        room: String,
        phone_work: String,
        phone_home: String,
        other: Option<Vec<String>>,
    },
    Simple {
        comment: String,
    },
}

impl Gecos {
    #[must_use]
    pub fn get_comment(&self) -> Option<&str> {
        match &self {
            Gecos::Simple { comment, .. } => Some(&comment),
            Gecos::Detail { .. } => None,
        }
    }
    #[must_use]
    pub fn get_full_name(&self) -> Option<&str> {
        match &self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { full_name, .. } => {
                if full_name.is_empty() {
                    None
                } else {
                    Some(&full_name)
                }
            }
        }
    }
    #[must_use]
    pub fn get_room(&self) -> Option<&str> {
        match &self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { room, .. } => {
                if room.is_empty() {
                    None
                } else {
                    Some(&room)
                }
            }
        }
    }
    #[must_use]
    pub fn get_phone_work(&self) -> Option<&str> {
        match &self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { phone_work, .. } => {
                if phone_work.is_empty() {
                    None
                } else {
                    Some(&phone_work)
                }
            }
        }
    }
    #[must_use]
    pub fn get_phone_home(&self) -> Option<&str> {
        match &self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { phone_home, .. } => {
                if phone_home.is_empty() {
                    None
                } else {
                    Some(&phone_home)
                }
            }
        }
    }
    #[must_use]
    pub const fn get_other(&self) -> Option<&Vec<String>> {
        match self {
            Gecos::Simple { .. } => None,
            Gecos::Detail { other, .. } => match other {
                None => None,
                Some(comments) => Some(comments),
            },
        }
    }
}

impl Display for Gecos {
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

impl TryFrom<String> for Gecos {
    type Error = UserLibError;
    fn try_from(source: String) -> std::result::Result<Self, Self::Error> {
        let vals: Vec<String> = source.split(',').map(ToString::to_string).collect();
        if vals.len() > 3 {
            Ok(Gecos::Detail {
                full_name: vals[0].clone(),
                room: vals[1].clone(),
                phone_work: vals[2].clone(),
                phone_home: vals[3].clone(),
                other: if vals.len() == 4 {
                    None
                } else {
                    Some(vals[4..].to_vec())
                },
            })
        } else if vals.len() == 1 {
            Ok(Gecos::Simple {
                comment: vals.get(0).unwrap().into(),
            })
        } else {
            panic!(format!("Could not parse this string: {}", source))
        }
    }
}

#[test]
fn test_parse_gecos() {
    // test if the Gecos field can be parsed and the resulting struct is populated correctly.
    let gcdetail = "Full Name,504,11345342,ä1-2312,myemail@test.com".to_string();
    let gcsimple = "A böring comment →".to_string();
    let gc_no_other = "systemd Network Management,,,".to_string();
    let res_detail = crate::Gecos::try_from(gcdetail).unwrap();
    let res_simple = crate::Gecos::try_from(gcsimple).unwrap();
    let res_no_other = crate::Gecos::try_from(gc_no_other).unwrap();
    match res_simple {
        crate::Gecos::Simple { comment } => assert_eq!(comment, "A böring comment →"),
        _ => unreachable!(),
    }
    match res_detail {
        crate::Gecos::Detail {
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
        crate::Gecos::Detail {
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
    let gcdetail = "Full Name,504,11345342,ä1-2312,myemail@test.com".to_string();
    let gcsimple = "A böring comment →".to_string();
    let gc_no_other = "systemd Network Management,,,".to_string();
    let res_detail = crate::Gecos::try_from(gcdetail).unwrap();
    let res_simple = crate::Gecos::try_from(gcsimple).unwrap();
    let res_no_other = crate::Gecos::try_from(gc_no_other).unwrap();
    assert_eq!(res_simple.get_comment(), Some("A böring comment →"));

    assert_eq!(res_detail.get_comment(), None);
    assert_eq!(res_detail.get_full_name(), Some("Full Name"));
    assert_eq!(res_detail.get_room(), Some("504"));
    assert_eq!(res_detail.get_phone_work(), Some("11345342"));
    assert_eq!(res_detail.get_phone_home(), Some("ä1-2312"));
    assert_eq!(
        res_detail.get_other(),
        Some(&vec!["myemail@test.com".to_string()])
    );

    assert_eq!(
        res_no_other.get_full_name(),
        Some("systemd Network Management")
    );
    assert_eq!(res_no_other.get_room(), None);
    assert_eq!(res_no_other.get_phone_work(), None);
    assert_eq!(res_no_other.get_phone_home(), None);
    assert_eq!(res_no_other.get_other(), None);
}
