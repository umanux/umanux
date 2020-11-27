extern crate umanux;
mod testfiles;

#[test]
fn test_delete_user_function() {
    use testfiles::Fixture;

    use std::fs;
    use umanux::api::UserDBWrite;
    use umanux::api::UserRead;

    let p = Fixture::copy("passwd");
    let s = Fixture::copy("shadow");
    let g = Fixture::copy("group");

    let pf = fs::read_to_string(&p.path).unwrap();

    let mf = umanux::Files {
        passwd: Some(p.path.clone()),
        shadow: Some(s.path),
        group: Some(g.path.clone()),
    };

    let mut db = umanux::UserDBLocal::load_files(mf).unwrap();

    let user_res: Result<umanux::User, umanux::UserLibError> = db.delete_user(
        umanux::api::DeleteUserArgs::builder()
            .username("teste")
            // .delete_home(umanux::api::DeleteHome::Delete)
            .build()
            .unwrap(),
    );
    let pf2 = fs::read_to_string(&p.path).unwrap();
    assert_eq!(user_res.unwrap().get_username().unwrap(), "teste");
    let pflines = pf.lines();
    let pflines2 = pf2.lines();
    for (l1, l2) in pflines.zip(pflines2.clone()) {
        if l1 != l2 {
            assert!(l1.starts_with("teste"));
            assert!(l2.starts_with("bergfried"));
            break;
        }
    }
    for line in pflines2 {
        assert!(!line.starts_with("teste"))
    }
    let gf2 = fs::read_to_string(&g.path).unwrap();
    let gflines2 = gf2.lines();
    for line in gflines2 {
        println!("{}", &line);
        assert!(!line.ends_with("teste"))
    }
}
