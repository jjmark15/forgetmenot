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

pub fn create_child_directories_n_levels_deep(starting_directory: &ChildPath, n: u8) -> ChildPath {
    let directory = nth_child_directory(starting_directory, n);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

fn nth_child_directory(starting_directory: &ChildPath, n: u8) -> ChildPath {
    let mut directory = starting_directory.child(".");

    for _i in 0..n {
        directory = directory.child("nested");
    }

    directory
}
