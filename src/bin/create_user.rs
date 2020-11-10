use std::path::PathBuf;

extern crate adduser;
use adduser::api::UserDBWrite;

fn main() {
    env_logger::init();

    let mf = adduser::Files {
        passwd: Some(PathBuf::from("./passwd")),
        shadow: Some(PathBuf::from("./shadow")),
        group: Some(PathBuf::from("./group")),
    };

    let mut db = adduser::UserDBLocal::load_files(mf).unwrap();

    let _user_res: Result<&adduser::User, adduser::UserLibError> = db.new_user(
        adduser::api::CreateUserArgs::builder()
            .username("teste")
            // .delete_home(adduser::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );

    let user = adduser::User::default()
        .username("fest".into())
        .shell_path("/bin/mash".into())
        .clone();

    println!("{}", user);

    //db.new_user().expect("failed to create the user");
}
