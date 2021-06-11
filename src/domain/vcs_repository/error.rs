#[derive(Debug, thiserror::Error)]
#[error("Repository is not initiated")]
pub(crate) struct VcsRepositoryNotInitiatedError;

#[derive(Debug, thiserror::Error)]
#[error("Repository does not exist at specified path")]
pub(crate) struct VcsRepositoryNotFoundError;
