pub trait UserDBRead {
    fn get_all_users(&self) -> Vec<&crate::User>;
    fn get_user_by_name(&self, name: &str) -> Option<&crate::User>;
    fn get_user_by_id(&self, uid: u32) -> Option<&crate::User>;
    fn get_all_groups(&self) -> Vec<&crate::Group>;
    fn get_group_by_name(&self, name: &str) -> Option<&crate::Group>;
    fn get_group_by_id(&self, name: u32) -> Option<&crate::Group>;
}

pub trait UserDBValidation {
    fn is_uid_valid_and_free(&self) -> bool;
    fn is_username_valid_and_free(&self) -> bool;
    fn is_gid_valid_and_free(&self) -> bool;
    fn is_groupname_valid_and_free(&self) -> bool;
}

pub trait UserDBWrite {
    fn delete_user(&self) -> Option<crate::User>;
    fn new_user(&self) -> Option<crate::User>;
    fn delete_group(&self) -> Option<crate::Group>;
    fn new_group(&self) -> Option<crate::Group>;
}

pub trait UserRead {
    fn get_username(&self) -> Option<crate::User>;
    fn get_uid(&self) -> Option<crate::User>;
    fn get_gid(&self) -> Option<crate::User>;
    fn get_password(&self) -> Option<crate::User>;
    fn get_gecos(&self) -> Option<crate::User>;
    fn get_home_dir(&self) -> Option<crate::User>;
    fn get_shell_path(&self) -> Option<crate::User>;
    fn get_full_name(&self) -> Option<String>;
    fn get_room(&self) -> Option<String>;
    fn get_phone_work(&self) -> Option<String>;
    fn get_phone_home(&self) -> Option<String>;
    fn get_other(&self) -> Option<Vec<String>>;
}

pub trait UserWrite {
    fn set_username(&self) -> Option<crate::User>;
    fn set_uid(&self) -> Option<crate::User>;
    fn set_gid(&self) -> Option<crate::User>;
    fn set_password(&self) -> Option<crate::User>;
    fn set_gecos(&self) -> Option<crate::User>;
    fn set_home_dir(&self) -> Option<crate::User>;
    fn set_shell_path(&self) -> Option<crate::User>;
    fn set_full_name(&self) -> Option<String>;
    fn set_room(&self) -> Option<String>;
    fn set_phone_work(&self) -> Option<String>;
    fn set_phone_home(&self) -> Option<String>;
    fn set_other(&self) -> Option<Vec<String>>;
}

pub trait GroupRead {
    fn get_groupname(&self) -> Option<crate::Group>;
    fn get_encrypted_password(&self) -> Option<crate::Group>;
    fn get_gid(&self) -> Option<crate::Group>;
    fn get_members(&self) -> Option<crate::Group>;
}

pub trait GroupWrite<T> {
    fn set_groupname(&self) -> Option<crate::Group>;
    fn set_password(&self) -> Option<crate::Group>;
    fn set_gid(&self) -> Option<crate::Group>;
    fn add_member(user: T) -> Option<crate::Group>;
    fn remove_member(user: T) -> Option<crate::Group>;
}
