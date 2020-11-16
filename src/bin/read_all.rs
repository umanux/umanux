extern crate umanux;
use umanux::api::UserDBRead;

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )])
    .unwrap();

    let db = umanux::UserDBLocal::load_files(umanux::Files::default()).unwrap();

    for u in db.get_all_users() {
        println!("{}", u);
    }
}
