use std::fs::File;
use std::path::PathBuf;

use crate::domain::{
    GetTestHistoryError, StoreTestHistoryError, TestHistory, TestHistoryRepository,
};
use crate::ports::persistence::filesystem::serde_test_history::SerdeTestHistories;

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

    fn read(&self) -> Result<SerdeTestHistories, ReadTestHistoryError> {
        let file = File::open(&Self::persistence_file_path())
            .map_err(|_err| ReadTestHistoryError::NotFound)?;
        let test_histories: SerdeTestHistories =
            serde_yaml::from_reader(file).map_err(|_err| ReadTestHistoryError::BadContent)?;
        Ok(test_histories)
    }

    fn write(&self, test_histories: &SerdeTestHistories) -> Result<(), WriteTestHistoryError> {
        let content = serde_yaml::to_string(test_histories).unwrap();
        std::fs::write(&Self::persistence_file_path(), content).map_err(|_e| WriteTestHistoryError)
    }
}

impl TestHistoryRepository for FilesystemTestHistoryRepositoryAdapter {
    fn get(&self, test_name: impl AsRef<str>) -> Result<TestHistory, GetTestHistoryError> {
        match self.read() {
            Ok(test_histories) => match test_histories.get(test_name) {
                None => Err(GetTestHistoryError::default()),
                Some(test_history) => Ok(test_history.clone().into()),
            },
            Err(_err) => Err(GetTestHistoryError::default()),
        }
    }

    fn store(
        &self,
        test_name: impl AsRef<str>,
        test_history: TestHistory,
    ) -> Result<(), StoreTestHistoryError> {
        let mut test_histories = self.read().map_err(|_e| StoreTestHistoryError::default())?;
        test_histories.insert(test_name.as_ref().to_string(), test_history.into());
        self.write(&test_histories)
            .map_err(|_e| StoreTestHistoryError::default())?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum ReadTestHistoryError {
    #[error("could not find test history on filesystem")]
    NotFound,
    #[error("could not read content of persisted test history")]
    BadContent,
}

#[derive(Debug, thiserror::Error)]
#[error("failed to write test history to persistence")]
struct WriteTestHistoryError;
