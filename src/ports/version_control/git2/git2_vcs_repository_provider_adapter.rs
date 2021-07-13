use std::path::PathBuf;

use crate::domain::{
    GetVcsRepositoryError, VcsRepository, VcsRepositoryHasNoCommitHistory,
    VcsRepositoryNotFoundError, VcsRepositoryNotInitiatedError, VcsRepositoryProvider, VcsVersion,
};

pub(crate) struct Git2VcsRepositoryProvidersAdapter {
    repository_path: PathBuf,
}

impl Git2VcsRepositoryProvidersAdapter {
    pub(crate) fn new(repository_path: PathBuf) -> Self {
        Git2VcsRepositoryProvidersAdapter { repository_path }
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
    fn get(&self) -> Result<VcsRepository, GetVcsRepositoryError> {
        if !self.repository_path.exists() {
            return Err(VcsRepositoryNotFoundError.into());
        }
        let git2_repo = git2::Repository::open(self.repository_path.as_path())
            .map_err(|_err| VcsRepositoryNotInitiatedError)?;
        let latest_version = self.get_latest_version(&git2_repo)?;
        Ok(VcsRepository::new(latest_version))
    }
}
