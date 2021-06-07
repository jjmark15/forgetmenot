use assert_fs::fixture::{ChildPath, PathChild, PathCreateDir};
use assert_fs::TempDir;

pub(crate) struct TestDirectoryManager {
    temp_directory_home: TempDir,
    test_directory_name: String,
}

impl TestDirectoryManager {
    pub(crate) fn new<S: ToString>(test_directory_name: S) -> Self {
        let temp_directory_home = assert_fs::TempDir::new().unwrap();
        let manager = TestDirectoryManager {
            temp_directory_home,
            test_directory_name: test_directory_name.to_string(),
        };
        manager.create_test_directory();
        manager
    }

    pub(crate) fn test_directory(&self) -> ChildPath {
        self.temp_directory_home.child(&self.test_directory_name)
    }

    fn create_test_directory(&self) {
        let path = self.test_directory();
        if !path.exists() {
            path.create_dir_all().unwrap();
        }
    }
}
