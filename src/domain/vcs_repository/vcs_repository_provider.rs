use crate::domain::{
    VcsRepository, VcsRepositoryHasNoCommitHistory, VcsRepositoryNotFoundError,
    VcsRepositoryNotInitiatedError,
};

pub(crate) trait VcsRepositoryProvider {
    fn get(&self) -> Result<VcsRepository, GetVcsRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetVcsRepositoryError {
    #[error(transparent)]
    NotInitiated(#[from] VcsRepositoryNotInitiatedError),
    #[error(transparent)]
    NotFound(#[from] VcsRepositoryNotFoundError),
    #[error(transparent)]
    NoCommitHistory(#[from] VcsRepositoryHasNoCommitHistory),
}
