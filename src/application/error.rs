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
