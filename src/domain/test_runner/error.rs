use crate::domain::test_provider::GetTestError;
use crate::domain::{ExecuteCommandError, TestNotFoundError};

#[derive(Debug, thiserror::Error)]
pub(crate) enum RunTestError {
    #[error(transparent)]
    TestNotFound(#[from] TestNotFoundError),
    #[error(transparent)]
    ExecutionFailure(#[from] ExecuteCommandError),
    #[error("failed to update test history")]
    UpdateTestHistory,
}

impl From<GetTestError> for RunTestError {
    fn from(get_test_error: GetTestError) -> Self {
        match get_test_error {
            GetTestError::TestNotFound(err) => err.into(),
        }
    }
}
