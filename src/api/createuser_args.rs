#![allow(clippy::default_trait_access)]
use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreateHome {
    Create,
    Skip,
    HomeFromDir { path: PathBuf },
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CreatePrimaryGroup {
    Create,
    Skip,
    CreateIfEmptyOrAdd,
}
#[derive(Debug, Builder, Eq, PartialEq)]
#[builder(public)]
#[builder(default)]
pub struct CreateUserArgs<'a> {
    pub username: &'a str,
    pub delete_home: CreateHome,
    pub delete_primary_group: CreatePrimaryGroup,
}

impl<'a> CreateUserArgs<'a> {
    #[must_use]
    pub fn builder() -> CreateUserArgsBuilder<'a> {
        CreateUserArgsBuilder::default()
    }
}

impl Default for CreateUserArgs<'_> {
    fn default() -> Self {
        Self {
            username: "defaultuser",
            delete_home: CreateHome::Create,
            delete_primary_group: CreatePrimaryGroup::CreateIfEmptyOrAdd,
        }
    }
}
