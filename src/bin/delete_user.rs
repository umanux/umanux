use std::path::PathBuf;

extern crate adduser;

use adduser::api::UserDBWrite;
use adduser::api::UserRead;

extern crate env_logger;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    let mf = adduser::Files {
        passwd: Some(PathBuf::from("./passwd")),
        shadow: Some(PathBuf::from("./shadow")),
        group: Some(PathBuf::from("./group")),
    };

    let mut db = adduser::UserDBLocal::load_files(mf).unwrap();

    let user_res: Result<adduser::User, adduser::UserLibError> = db.delete_user(
        adduser::api::NewUserArgs::builder()
            .username("teste")
            // .delete_home(adduser::api::DeleteHome::Delete)
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
