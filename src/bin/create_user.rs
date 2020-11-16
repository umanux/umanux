use std::path::PathBuf;

extern crate clap;
use clap::{App, Arg};

extern crate umanux;
use umanux::{api::UserDBWrite, UserLibError};

fn main() -> Result<(), UserLibError> {
    env_logger::init();
    let matches = App::new("Create a new linux user")
        .version("0.1.0")
        .author("Franz Dietrich <dietrich@teilgedanken.de>")
        .about("Create a linux user do not use this in production (yet)")
        .arg(
            Arg::new("username")
                .value_name("USERNAME")
                .about("the new users name")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("passwd")
                .short('p')
                .long("passwd")
                .value_name("FILE")
                .about("The passwd file")
                .default_value("/etc/passwd")
                .takes_value(true),
        )
        .arg(
            Arg::new("shadow")
                .short('s')
                .long("shadow")
                .value_name("FILE")
                .about("The shadow file")
                .default_value("/etc/shadow")
                .takes_value(true),
        )
        .arg(
            Arg::new("group")
                .short('g')
                .long("group")
                .value_name("FILE")
                .about("The group file")
                .default_value("/etc/group")
                .takes_value(true),
        )
        .get_matches();

    let mf = umanux::Files {
        passwd: Some(PathBuf::from(matches.value_of("passwd").unwrap())),
        shadow: Some(PathBuf::from(matches.value_of("shadow").unwrap())),
        group: Some(PathBuf::from(matches.value_of("group").unwrap())),
    };

    let mut db = umanux::UserDBLocal::load_files(mf).unwrap();

    match db.new_user(
        umanux::api::CreateUserArgs::builder()
            .username(matches.value_of("username").unwrap())
            // .delete_home(umanux::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
