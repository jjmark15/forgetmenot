use std::collections::HashMap;
use std::rc::Rc;

use crate::domain::checklist::Checklist;
use crate::domain::{
    DetermineCurrentProjectVersionError, GetChecklistError, GetTestHistoryError,
    ReadTestHistoryError, Test, TestHistoryRepository, VcsRepositoryProvider, VcsVersion,
};

pub(crate) trait ChecklistService {
    fn get(&self, tests: Vec<&Test>) -> Result<Checklist, GetChecklistError>;
}

pub(crate) struct VcsVersionChecklistService<VRP, THR>
where
    VRP: VcsRepositoryProvider,
    THR: TestHistoryRepository,
{
    vcs_repository_provider: Rc<VRP>,
    test_history_repository: Rc<THR>,
}

impl<VRP, THR> VcsVersionChecklistService<VRP, THR>
where
    VRP: VcsRepositoryProvider,
    THR: TestHistoryRepository,
{
    pub(crate) fn new(vcs_repository_provider: Rc<VRP>, test_history_repository: Rc<THR>) -> Self {
        VcsVersionChecklistService {
            vcs_repository_provider,
            test_history_repository,
        }
    }

    fn has_been_run_against_current_project_version(
        &self,
        test_name: impl AsRef<str>,
        vcs_version: &VcsVersion,
    ) -> Result<bool, ReadTestHistoryError> {
        let has_been_run = match self.test_history_repository.get(test_name) {
            Ok(test_history) => test_history
                .result_for(vcs_version)
                .map(|result| result.is_success())
                .unwrap_or(false),
            Err(err) => match err {
                GetTestHistoryError::NotFound => false,
            },
        };

        Ok(has_been_run)
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
            .get()
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
