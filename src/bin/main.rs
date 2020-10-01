extern crate adduser;

use adduser::passwd::Passwd;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    simplelog::CombinedLogger::init(vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Warn,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
    )])
    .unwrap();
    let file = File::open("/etc/passwd").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        println!("{}", Passwd::new_from_string(&line).unwrap());
    }

    // let pwd = Passwd::default();
    // let pwd2 =
    //     Passwd::new_from_string("howdy:notencrypted:1001:1001:not done:/home/test:/bin/bash");
    // println!("Test struct: {}", pwd);

    // assert_eq!(pwd, pwd2.unwrap())
}
