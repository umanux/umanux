use std::path::PathBuf;

extern crate adduser;

extern crate env_logger;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    use adduser::api::UserDBWrite;
    use adduser::api::UserRead;

    let mf = adduser::Files {
        passwd: Some(PathBuf::from("./passwd")),
        shadow: Some(PathBuf::from("./shadow")),
        group: Some(PathBuf::from("./group")),
    };

    let mut db = adduser::UserDBLocal::load_files(mf).unwrap();

    let user_res: Result<adduser::User, adduser::UserLibError> = db.delete_user("teste");
    match user_res {
        Ok(u) => info!(
            "The user <{}> has been deleted! ",
            u.get_username().unwrap()
        ),
        Err(e) => error!("Failed to delete the user: {}", e),
    }
}
