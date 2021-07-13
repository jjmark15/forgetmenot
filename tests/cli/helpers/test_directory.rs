use std::path::Path;

use assert_fs::fixture::{ChildPath, PathChild};
use assert_fs::TempDir;

pub(crate) struct TestDirectoryManager {
    temp_directory_root: TempDir,
    test_directory_name: String,
}

impl TestDirectoryManager {
    pub(crate) fn new<S: ToString>(test_directory_name: S) -> Self {
        TestDirectoryManager {
            temp_directory_root: assert_fs::TempDir::new().unwrap(),
            test_directory_name: test_directory_name.to_string(),
        }
    }

    pub(crate) fn test_directory(&self) -> ChildPath {
        let test_directory = self.temp_directory_root.child(&self.test_directory_name);
        self.create_directory_if_does_not_exist(&test_directory);
        test_directory
    }

    pub(crate) fn home_directory(&self) -> ChildPath {
        let home_directory_path = self.test_directory().child("home");
        self.create_directory_if_does_not_exist(&home_directory_path);
        home_directory_path
    }

    fn create_directory_if_does_not_exist(&self, directory: impl AsRef<Path>) {
        if !directory.as_ref().exists() {
            std::fs::create_dir_all(directory).unwrap();
        }
    }
}
