extern crate adduser;

use adduser::NewFromString;
use adduser::Shadow;
use adduser::User;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )])
    .unwrap();
    let file: File = File::open("/etc/passwd").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line.unwrap();
        println!("{}", User::new_from_string(line).unwrap());
    }

    let line = "test:!!$6$/RotIe4VZzzAun4W$7YUONvru1rDnllN5TvrnOMsWUD5wSDUPAD6t6/Xwsr/0QOuWF3HcfAhypRkGa8G1B9qqWV5kZSnCb8GKMN9N61:18260:0:99999:7:::".to_string();
    assert_eq!(
        format!("{}", Shadow::new_from_string(line.clone()).unwrap()),
        line
    );

    // let pwd = User::default();
    // let pwd2 =
    //     User::new_from_string("howdy:notencrypted:1001:1001:not done:/home/test:/bin/bash");
    // println!("Test struct: {}", pwd);

    // assert_eq!(pwd, pwd2.unwrap())
}
