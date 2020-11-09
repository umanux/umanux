#![warn(
    clippy::all,
    //clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
//#![allow(clippy::non_ascii_literal)]
#![allow(clippy::missing_errors_doc)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_builder;

extern crate log;

pub mod api;
pub mod error;
pub mod group;
pub mod user;
pub mod userlib;
pub use error::UserLibError;
pub use group::Group;
pub use user::gecos_fields::Gecos;
pub use user::passwd_fields::{
    EncryptedPassword, Gid, HomeDir, Password, ShellPath, Uid, Username,
};
pub use user::shadow_fields::Shadow;
pub use user::User;
pub use userlib::{files::Files, NewFromString, UserDBLocal};
