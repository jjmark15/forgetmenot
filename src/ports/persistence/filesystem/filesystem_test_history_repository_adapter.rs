use std::fs::File;
use std::path::PathBuf;

use crate::domain::{GetTestHistoryError, TestHistory, TestHistoryRepository};
use crate::ports::persistence::filesystem::serde_test_history::SerdeTestHistory;

pub(crate) struct FilesystemTestHistoryRepositoryAdapter {}

impl FilesystemTestHistoryRepositoryAdapter {
    pub(crate) fn new() -> Self {
        FilesystemTestHistoryRepositoryAdapter {}
    }

    fn persistence_file_path() -> PathBuf {
        dirs::home_dir()
            .unwrap()
            .join(".forgetmenot")
            .join("test_history.yml")
    }

    fn read(&self) -> Result<SerdeTestHistory, ReadTestHistoryError> {
        let file = File::open(&Self::persistence_file_path())
            .map_err(|_err| ReadTestHistoryError::NotFound)?;
        let test_result: SerdeTestHistory =
            serde_yaml::from_reader(file).map_err(|_err| ReadTestHistoryError::BadContent)?;
        Ok(test_result)
    }
}

impl TestHistoryRepository for FilesystemTestHistoryRepositoryAdapter {
    fn get(&self, _test_name: impl AsRef<str>) -> Result<TestHistory, GetTestHistoryError> {
        match self.read() {
            Ok(test_history) => Ok(test_history.into()),
            Err(err) => match err {
                ReadTestHistoryError::NotFound => Ok(TestHistory::default()),
                ReadTestHistoryError::BadContent => Err(GetTestHistoryError::default()),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ReadTestHistoryError {
    #[error("could not find test history on filesystem")]
    NotFound,
    #[error("could not read content of persisted test history")]
    BadContent,
}
