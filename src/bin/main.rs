extern crate adduser;

use adduser::passwd::Passwd;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {
    let mut file = File::open("/etc/passwd").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", Passwd::new_from_string(&line.unwrap()).unwrap());
    }

    // let pwd = Passwd::default();
    // let pwd2 =
    //     Passwd::new_from_string("howdy:notencrypted:1001:1001:not done:/home/test:/bin/bash");
    // println!("Test struct: {}", pwd);

    // assert_eq!(pwd, pwd2.unwrap())
}
