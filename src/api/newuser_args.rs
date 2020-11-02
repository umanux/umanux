use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DeleteHome {
    Delete,
    Keep,
    Archive { path: PathBuf },
}
#[derive(Debug, Clone)]
pub enum DeletePrimaryGroup {
    Delete,
    Keep,
    DeleteIfEmpty,
}

#[derive(Debug, Builder)]
#[builder(public)]
pub struct NewUserArgs<'a> {
    pub username: &'a str,
    pub delete_home: DeleteHome,
    pub delete_primary_group: DeletePrimaryGroup,
}

impl<'a> NewUserArgs<'a> {
    pub fn builder() -> NewUserArgsBuilder<'a> {
        NewUserArgsBuilder::default()
    }
}

impl Default for NewUserArgs<'_> {
    fn default() -> Self {
        Self {
            username: "defaultuser",
            delete_home: DeleteHome::Keep,
            delete_primary_group: DeletePrimaryGroup::DeleteIfEmpty,
        }
    }
}
