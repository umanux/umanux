extern crate adduser;
extern crate test_bin;
mod testfiles;

#[test]
fn test_create_user_function() {
    use testfiles::Fixture;

    use adduser::api::UserDBWrite;
    use adduser::api::UserRead;
    use std::fs;

    let p = Fixture::copy("passwd");
    let s = Fixture::copy("shadow");
    let g = Fixture::copy("group");

    let pf = fs::read_to_string(&p.path).unwrap();

    let mf = adduser::Files {
        passwd: Some(p.path.clone()),
        shadow: Some(s.path),
        group: Some(g.path),
    };

    let mut db = adduser::UserDBLocal::load_files(mf).unwrap();

    let user_res: Result<&adduser::User, adduser::UserLibError> = db.new_user(
        adduser::api::CreateUserArgs::builder()
            .username("test2")
            // .delete_home(adduser::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );
    let password_file_string = fs::read_to_string(&p.path).unwrap();
    let shadow_file_string = fs::read_to_string(&p.path).unwrap();
    assert_eq!(user_res.unwrap().get_username().unwrap(), "test2");
    let pflines = pf.lines();
    let pflines2 = password_file_string.lines();
    for (l1, l2) in pflines.zip(pflines2) {
        dbg!(l1, l2);
        assert!(l1 == l2);
    }
    assert!(password_file_string
        .lines()
        .last()
        .unwrap()
        .starts_with("test2"));
    assert!(shadow_file_string
        .lines()
        .last()
        .unwrap()
        .starts_with("test2"));
}
#[test]
fn test_create_user_binary() {
    use testfiles::Fixture;

    use std::fs;

    let p = Fixture::copy("passwd");
    let s = Fixture::copy("shadow");
    let g = Fixture::copy("group");

    //dbg!(&p, &s, &g);

    let passwd_string = fs::read_to_string(&p.path).unwrap();
    let passwd_lines = passwd_string.lines();
    let shadow_string = fs::read_to_string(&s.path).unwrap();
    let shadow_lines = shadow_string.lines();

    let out = test_bin::get_test_bin("create_user")
        .args(&[
            "testuser3",
            "-p",
            p.path.to_str().unwrap(),
            "-s",
            s.path.to_str().unwrap(),
            "-g",
            g.path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run the command");
    println!(
        "The output after running: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    println!(
        "The error after running: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&out.stdout), "");
    assert_eq!(String::from_utf8_lossy(&out.stderr), "");

    let passwd_string_after = fs::read_to_string(&p.path).unwrap();
    let passwd_lines_after = passwd_string_after.lines();
    let shadow_string_after = fs::read_to_string(&s.path).unwrap();
    let shadow_lines_after = shadow_string_after.lines();
    for (l1, l2) in passwd_lines.zip(passwd_lines_after) {
        //dbg!(l1, l2);
        assert!(l1 == l2);
    }
    assert_eq!(
        passwd_string_after
            .lines()
            .last()
            .unwrap()
            .starts_with("testuser3"),
        true
    );
    //dbg!(&shadow_string_after);
    for (l1, l2) in shadow_lines.zip(shadow_lines_after) {
        //dbg!(l1, l2);
        assert!(l1 == l2);
    }
    assert_eq!(
        shadow_string_after
            .lines()
            .last()
            .unwrap()
            .starts_with("testuser3"),
        true
    );
}
