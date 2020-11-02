pub mod newuser_args;

pub use newuser_args::{DeleteHome, DeletePrimaryGroup, NewUserArgs};
pub trait UserDBRead {
    fn get_all_users(&self) -> Vec<&crate::User>;
    fn get_user_by_name(&self, name: &str) -> Option<&crate::User>;
    fn get_user_by_id(&self, uid: u32) -> Option<&crate::User>;
    fn get_all_groups(&self) -> Vec<&crate::Group>;
    fn get_group_by_name(&self, name: &str) -> Option<&crate::Group>;
    fn get_group_by_id(&self, name: u32) -> Option<&crate::Group>;
}

pub trait UserDBValidation {
    fn is_uid_valid_and_free(&self, uid: u32) -> bool;
    fn is_username_valid_and_free(&self, name: &str) -> bool;
    fn is_gid_valid_and_free(&self, gid: u32) -> bool;
    fn is_groupname_valid_and_free(&self, name: &str) -> bool;
}

pub trait UserDBWrite {
    fn delete_user(
        &mut self,
        params: newuser_args::NewUserArgs,
    ) -> Result<crate::User, crate::UserLibError>;
    fn new_user(
        &mut self, /*
                   username: String,
                   enc_password: String,
                   uid: u32,
                   gid: u32,
                   full_name: String,
                   room: String,
                   phone_work: String,
                   phone_home: String,
                   other: Option<Vec<String>>,
                   home_dir: String,
                   shell_path: String,*/
    ) -> Result<&crate::User, crate::UserLibError>;
    fn delete_group(&mut self, group: &crate::Group) -> Result<(), crate::UserLibError>;
    fn new_group(&mut self) -> Result<&crate::Group, crate::UserLibError>;
}

pub trait UserRead {
    fn get_username(&self) -> Option<&str>;
    fn get_uid(&self) -> u32;
    fn get_gid(&self) -> u32;
    fn get_password(&self) -> Option<&str>;
    fn get_gecos(&self) -> Option<&crate::Gecos>;
    fn get_home_dir(&self) -> Option<&str>;
    fn get_shell_path(&self) -> Option<&str>;
    fn get_full_name(&self) -> Option<&str>;
    fn get_room(&self) -> Option<&str>;
    fn get_phone_work(&self) -> Option<&str>;
    fn get_phone_home(&self) -> Option<&str>;
    fn get_other(&self) -> Option<&Vec<String>>;
}

pub trait UserWrite {
    fn set_username(&self, username: String);
    fn set_uid(&self, uid: u32);
    fn set_gid(&self, gid: u32);
    fn set_password(&self, password: String);
    fn set_gecos(&self, gecos: crate::Gecos);
    fn set_home_dir(&self, home_dir: String);
    fn set_shell_path(&self, shell_path: String);
    fn set_full_name(&self, full_name: String);
    fn set_room(&self, room: String);
    fn set_phone_work(&self, phone_work: String);
    fn set_phone_home(&self, phone_home: String);
    fn set_other(&self, other: Option<Vec<String>>);
}

pub trait GroupRead {
    fn get_groupname(&self) -> Option<&str>;
    fn get_encrypted_password(&self) -> Option<&str>;
    fn get_gid(&self) -> Option<u32>;
    fn get_member_names(&self) -> Option<Vec<&str>>;
}

pub trait GroupWrite<T> {
    fn set_groupname(&self) -> Option<crate::Group>;
    fn set_password(&self) -> Option<crate::Group>;
    fn set_gid(&self) -> Option<crate::Group>;
    fn add_member(user: T) -> Option<crate::Group>;
    fn remove_member(user: T) -> Option<crate::Group>;
}
