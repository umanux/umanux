use std::path::PathBuf;

extern crate umanux;

use umanux::api::UserDBWrite;
use umanux::api::UserRead;

extern crate env_logger;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    let mf = umanux::Files {
        passwd: Some(PathBuf::from("./passwd")),
        shadow: Some(PathBuf::from("./shadow")),
        group: Some(PathBuf::from("./group")),
    };

    let mut db = umanux::UserDBLocal::load_files(mf).unwrap();

    let user_res: Result<umanux::User, umanux::UserLibError> = db.delete_user(
        umanux::api::DeleteUserArgs::builder()
            .username("teste")
            // .delete_home(umanux::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );
    match user_res {
        Ok(u) => info!(
            "The user <{}> has been deleted! ",
            u.get_username().unwrap()
        ),
        Err(e) => error!("Failed to delete the user: {}", e),
    }
}
