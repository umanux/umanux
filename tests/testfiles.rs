use std::{env, path::Path};
use std::{fs, path::PathBuf};

use tempfile::TempDir;

use std::ops::Deref;

pub struct Fixture {
    pub path: PathBuf,
    pub source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    #[must_use]
    pub fn blank(fixture_filename: &str) -> Self {
        // First, figure out the right file in `tests/fixtures/`:
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        // The "real" path of the file is going to be under a temporary directory:
        let tempdir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        Self {
            _tempdir: tempdir,
            source,
            path,
        }
    }
    #[must_use]
    pub fn copy(fixture_filename: &str) -> Self {
        let fixture = Self::blank(fixture_filename);
        fs::copy(&fixture.source, &fixture.path).unwrap();
        fixture
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
