extern crate adduser;
mod testfiles;

#[test]
fn test_test() {
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

    let user_res: Result<adduser::User, adduser::UserLibError> = db.delete_user(
        adduser::api::NewUserArgs::builder()
            .username("teste")
            // .delete_home(adduser::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );
    let pf2 = fs::read_to_string(&p.path).unwrap();
    assert_eq!(user_res.unwrap().get_username().unwrap(), "teste");
    let pflines = pf.lines();
    let pflines2 = pf2.lines();
    for (l1, l2) in pflines.zip(pflines2) {
        if l1 != l2 {
            dbg!(l1, l2);
            assert!(l1.starts_with("teste"));
            assert!(l2.starts_with("bergfried"));
            break;
        }
    }
}
