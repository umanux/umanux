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

    let user = adduser::User::default()
        .username("fest".into())
        .shell_path("/bin/mash".into())
        .clone();

    println!("{}", user);

    //db.new_user().expect("failed to create the user");
}
