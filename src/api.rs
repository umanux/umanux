trait UserDBRead {
    fn get_all_users(&self) -> Vec<crate::User>;
    fn get_user_by_name(&self, name: &str) -> Option<crate::User>;
    fn get_user_by_id(&self, uid: u64) -> Option<crate::User>;
    fn get_all_groups(&self) -> Vec<crate::Group>;
    fn get_group_by_name(&self) -> Option<crate::Group>;
    fn get_group_by_id(&self) -> Option<crate::Group>;
}

trait UserDBValidation {
    fn is_uid_valid_and_free(&self) -> bool;
    fn is_username_valid_and_free(&self) -> bool;
    fn is_gid_valid_and_free(&self) -> bool;
    fn is_groupname_valid_and_free(&self) -> bool;
}

trait UserDBWrite {
    fn set_user(&self) -> Option<crate::User>;
    fn new_user(&self) -> Option<crate::User>;
    fn set_group(&self) -> Option<crate::Group>;
    fn new_group(&self) -> Option<crate::Group>;
}

trait UserRead {
    fn get_username(&self) -> Option<crate::User>;
    fn get_uid(&self) -> Option<crate::User>;
    fn get_gid(&self) -> Option<crate::User>;
    // …
}

trait UserWrite {}

trait GroupRead {}

trait GroupWrite {}
