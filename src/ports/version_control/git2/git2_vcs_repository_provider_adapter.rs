use std::path::Path;

use crate::domain::{
    GetVcsRepositoryError, VcsRepository, VcsRepositoryHasNoCommitHistory,
    VcsRepositoryNotFoundError, VcsRepositoryNotInitiatedError, VcsRepositoryProvider, VcsVersion,
};

pub(crate) struct Git2VcsRepositoryProvidersAdapter {}

impl Git2VcsRepositoryProvidersAdapter {
    pub(crate) fn new() -> Self {
        Git2VcsRepositoryProvidersAdapter {}
    }

    fn get_latest_version(
        &self,
        _repo: &git2::Repository,
    ) -> Result<VcsVersion, VcsRepositoryHasNoCommitHistory> {
        let commit = last_git_commit::LastGitCommit::new()
            .build()
            .map_err(|_err| VcsRepositoryHasNoCommitHistory)?;
        let commit_id = commit.id();
        Ok(VcsVersion::new(commit_id.long()))
    }
}

impl VcsRepositoryProvider for Git2VcsRepositoryProvidersAdapter {
    fn get(
        &self,
        vcs_repository_path: impl AsRef<Path>,
    ) -> Result<VcsRepository, GetVcsRepositoryError> {
        if !vcs_repository_path.as_ref().exists() {
            return Err(VcsRepositoryNotFoundError.into());
        }
        let git2_repo = git2::Repository::open(vcs_repository_path.as_ref())
            .map_err(|_err| VcsRepositoryNotInitiatedError)?;
        let latest_version = self.get_latest_version(&git2_repo)?;
        Ok(VcsRepository::new(latest_version))
    }
}
