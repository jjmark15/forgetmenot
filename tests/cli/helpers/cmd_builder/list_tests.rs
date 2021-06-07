use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;

use crate::helpers::cmd_builder::subcommand::SubcommandBase;
use crate::helpers::SubcommandBuilder;

pub(crate) struct ListTestsCommandBuilder {
    subcommand_base: SubcommandBase,
    config_path: Option<PathBuf>,
}

impl ListTestsCommandBuilder {
    pub(crate) fn new(subcommand_base: SubcommandBase) -> Self {
        ListTestsCommandBuilder {
            subcommand_base,
            config_path: None,
        }
    }

    pub(crate) fn with_config<P: AsRef<Path>>(mut self, config_path: P) -> Self {
        self.config_path = Some(config_path.as_ref().to_path_buf());
        self
    }
}

impl SubcommandBuilder for ListTestsCommandBuilder {
    fn assert(self) -> Assert {
        let mut cmd = self.subcommand_base.cmd();
        cmd.arg("list");
        if let Some(config_path) = self.config_path {
            cmd.args(&["--config-path", config_path.as_os_str().to_str().unwrap()]);
        }

        cmd.assert()
    }

    fn with_current_directory<P: AsRef<Path>>(mut self, directory: P) -> Self {
        self.subcommand_base.with_current_directory(directory);
        self
    }
}
