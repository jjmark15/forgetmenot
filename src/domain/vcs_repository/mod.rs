pub(crate) use error::*;
pub(crate) use vcs_repository_provider::*;
pub(crate) use version::*;

mod error;
mod vcs_repository_provider;
mod version;

pub(crate) struct VcsRepository {
    version: VcsVersion,
}

impl VcsRepository {
    pub(crate) fn new(version: VcsVersion) -> Self {
        VcsRepository { version }
    }

    pub(crate) fn version(&self) -> &VcsVersion {
        &self.version
    }
}
