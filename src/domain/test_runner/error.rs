use crate::domain::test_provider::GetTestError;
use crate::domain::{ExecuteCommandError, StoreTestHistoryError, TestNotFoundError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum RunTestError {
    #[error(transparent)]
    TestNotFound(#[from] TestNotFoundError),
    #[error(transparent)]
    ExecutionFailure(#[from] ExecuteCommandError),
    #[error(transparent)]
    UpdateTestHistory(#[from] UpdateTestHistoryError),
}

impl From<GetTestError> for RunTestError {
    fn from(get_test_error: GetTestError) -> Self {
        match get_test_error {
            GetTestError::TestNotFound(err) => err.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum UpdateTestHistoryError {
    #[error("failed to get current project version")]
    GetCurrentVersion,
    #[error(transparent)]
    StoreTestHistory(#[from] StoreTestHistoryError),
}
