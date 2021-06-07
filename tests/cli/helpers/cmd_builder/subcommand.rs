use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub(crate) trait SubcommandBuilder {
    fn assert(self) -> Assert;

    fn with_current_directory<P: AsRef<Path>>(self, directory: P) -> Self;
}

pub(crate) struct SubcommandBase {
    current_directory: Option<PathBuf>,
    cmd: Command,
}

impl SubcommandBase {
    pub(crate) fn new(cmd: Command) -> Self {
        SubcommandBase {
            cmd,
            current_directory: None,
        }
    }

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
