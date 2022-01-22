use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;
use assert_cmd::Command;
use derive_new::new;

pub(crate) trait SubcommandBuilder {
    fn assert(self) -> Assert;

    fn with_current_directory<P: AsRef<Path>>(self, directory: P) -> Self;
}

#[derive(new)]
pub(crate) struct SubcommandBase {
    cmd: Command,
    #[new(default)]
    current_directory: Option<PathBuf>,
}

impl SubcommandBase {
    pub(crate) fn with_current_directory<P: AsRef<Path>>(&mut self, directory: P) {
        self.current_directory = Some(directory.as_ref().to_path_buf());
    }

    pub(crate) fn cmd(mut self) -> Command {
        if let Some(current_directory) = self.current_directory {
            self.cmd.current_dir(current_directory);
        }
        self.cmd
    }
}
