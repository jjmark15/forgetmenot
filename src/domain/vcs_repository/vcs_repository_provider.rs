use std::path::Path;

use crate::domain::{VcsRepository, VcsRepositoryNotFoundError, VcsRepositoryNotInitiatedError};

pub(crate) trait VcsRepositoryProvider {
    fn get(
        &self,
        vcs_repository_path: impl AsRef<Path>,
    ) -> Result<VcsRepository, GetVcsRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetVcsRepositoryError {
    #[error(transparent)]
    NotInitiated(#[from] VcsRepositoryNotInitiatedError),
    #[error(transparent)]
    NotFound(#[from] VcsRepositoryNotFoundError),
}
