use crate::domain::GetTestError;

#[derive(Debug, thiserror::Error)]
#[error("{cause}")]
pub(crate) struct RunTestError {
    pub(crate) cause: crate::domain::RunTestError,
}

impl From<crate::domain::RunTestError> for RunTestError {
    fn from(cause: crate::domain::RunTestError) -> Self {
        RunTestError { cause }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{cause}")]
pub(crate) struct DescribeTestError {
    #[from]
    pub(crate) cause: crate::domain::TestNotFoundError,
}

impl From<crate::domain::GetTestError> for DescribeTestError {
    fn from(err: GetTestError) -> Self {
        match err {
            GetTestError::TestNotFound(not_found) => not_found.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{cause}")]
pub(crate) struct GetChecklistError {
    #[from]
    pub(crate) cause: crate::domain::GetChecklistError,
}
