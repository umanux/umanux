extern crate adduser;
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
    let pf2 = fs::read_to_string(&p.path).unwrap();
    assert_eq!(user_res.unwrap().get_username().unwrap(), "test2");
    let pflines = pf.lines();
    let pflines2 = pf2.lines();
    for (l1, l2) in pflines.zip(pflines2) {
        dbg!(l1, l2);
        assert!(l1 == l2);
    }
    assert!(pf2.lines().last().unwrap().starts_with("test2"));
}
