extern crate adduser;

extern crate env_logger;

fn main() {
    env_logger::init();

    use adduser::api::UserDBWrite;

    let mut db = adduser::UserDBLocal::load_files(adduser::Files::default());

    let user = db.delete_user("teste").expect("failed to delete the user");
    println!("The user {} has been deleted!", user);
}
