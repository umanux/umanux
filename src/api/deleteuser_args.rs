#![allow(clippy::default_trait_access)]
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeleteHome {
    Delete,
    Keep,
    Archive { path: PathBuf },
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DeletePrimaryGroup {
    Delete,
    Keep,
    DeleteIfEmpty,
}
#[derive(Debug, Builder, Eq, PartialEq)]
#[builder(public)]
#[builder(default)]
pub struct DeleteUserArgs<'a> {
    pub username: &'a str,
    pub delete_home: DeleteHome,
    pub delete_primary_group: DeletePrimaryGroup,
}

impl<'a> DeleteUserArgs<'a> {
    #[must_use]
    pub fn builder() -> DeleteUserArgsBuilder<'a> {
        DeleteUserArgsBuilder::default()
    }
}

impl Default for DeleteUserArgs<'_> {
    fn default() -> Self {
        Self {
            username: "defaultuser",
            delete_home: DeleteHome::Keep,
            delete_primary_group: DeletePrimaryGroup::DeleteIfEmpty,
        }
    }
}
