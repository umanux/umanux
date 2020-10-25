extern crate adduser;

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )])
    .unwrap();
    use adduser::api::UserDBWrite;

    let mut db = adduser::UserDBLocal::load_files(adduser::Files::default());

    let user = db.delete_user("teste").expect("failed to delete the user");
    println!("The user {} has been deleted!", user);
}
