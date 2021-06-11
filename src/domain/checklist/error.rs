#[derive(Debug, thiserror::Error)]
pub(crate) enum GetChecklistError {
    #[error(transparent)]
    CurrentVersion(#[from] DetermineCurrentProjectVersionError),
    #[error(transparent)]
    TestHistory(#[from] ReadTestHistoryError),
}

#[derive(Default, Debug, thiserror::Error)]
#[error("failed to determine current project version")]
pub(crate) struct DetermineCurrentProjectVersionError;

#[derive(Default, Debug, thiserror::Error)]
#[error("failed to read project test history")]
pub(crate) struct ReadTestHistoryError;
