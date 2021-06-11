use std::collections::HashMap;
use std::path::PathBuf;

use crate::domain::checklist::Checklist;
use crate::domain::{
    DetermineCurrentProjectVersionError, GetChecklistError, ReadTestHistoryError, Test,
    TestHistoryRepository, VcsRepositoryProvider, VcsVersion,
};

pub(crate) trait ChecklistService {
    fn get(&self, tests: Vec<&Test>) -> Result<Checklist, GetChecklistError>;
}

pub(crate) struct VcsVersionChecklistService<VRP, THR>
where
    VRP: VcsRepositoryProvider,
    THR: TestHistoryRepository,
{
    vcs_repository_provider: VRP,
    vcs_repository_path: PathBuf,
    test_history_provider: THR,
}

impl<VRP, THR> VcsVersionChecklistService<VRP, THR>
where
    VRP: VcsRepositoryProvider,
    THR: TestHistoryRepository,
{
    pub(crate) fn new(
        vcs_repository_provider: VRP,
        vcs_repository_path: PathBuf,
        test_history_provider: THR,
    ) -> Self {
        VcsVersionChecklistService {
            vcs_repository_provider,
            vcs_repository_path,
            test_history_provider,
        }
    }

    fn has_been_run_against_current_project_version(
        &self,
        test_name: impl AsRef<str>,
        vcs_version: &VcsVersion,
    ) -> Result<bool, ReadTestHistoryError> {
        let test_history = self
            .test_history_provider
            .get(test_name)
            .map_err(|_err| ReadTestHistoryError::default())?;

        let checked = test_history
            .result_for(vcs_version)
            .map(|result| result.is_success())
            .unwrap_or(false);

        Ok(checked)
    }
}

impl<VRP, THR> ChecklistService for VcsVersionChecklistService<VRP, THR>
where
    VRP: VcsRepositoryProvider,
    THR: TestHistoryRepository,
{
    fn get(&self, tests: Vec<&Test>) -> Result<Checklist, GetChecklistError> {
        let repository = self
            .vcs_repository_provider
            .get(&self.vcs_repository_path)
            .map_err(|_| DetermineCurrentProjectVersionError::default())?;

        let checklist_entries = tests
            .into_iter()
            .map(|test| {
                self.has_been_run_against_current_project_version(test.name(), repository.version())
                    .map(|checked| (test.name().clone(), checked))
            })
            .collect::<Result<HashMap<String, bool>, ReadTestHistoryError>>()?;

        Ok(Checklist::new(checklist_entries))
    }
}
