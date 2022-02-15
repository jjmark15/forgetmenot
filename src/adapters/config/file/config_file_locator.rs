use std::path::{Path, PathBuf};

#[derive(derive_new::new)]
pub(crate) struct ConfigFileLocator {}

impl ConfigFileLocator {
    pub(crate) fn locate(&self, starting_directory: &Path) -> Result<PathBuf, NoConfigFound> {
        let file_name = "forgetmenot.yml";
        match find_path_containing_recursive(starting_directory, file_name) {
            None => Err(NoConfigFound),
            Some(directory_containing_config) => Ok(directory_containing_config.join(file_name)),
        }
    }
}

fn find_path_containing_recursive(dir: &Path, file_name: &str) -> Option<PathBuf> {
    let found = find_file_in_dir(&dir, file_name);

    if found {
        Some(dir.to_path_buf())
    } else {
        match dir.parent() {
            Some(parent) => find_path_containing_recursive(&parent.to_path_buf(), file_name),
            None => None,
        }
    }
}

fn find_file_in_dir(dir: &Path, file_name: &str) -> bool {
    dir.join(file_name).exists()
}

#[derive(Debug, thiserror::Error)]
#[error("could not find a config")]
pub(crate) struct NoConfigFound;
