use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;

use crate::helpers::cmd_builder::subcommand::{SubcommandBase, SubcommandBuilder};

pub(crate) struct RunTestCommandBuilder {
    test_name: String,
    config_path: Option<PathBuf>,
    subcommand_base: SubcommandBase,
}

impl RunTestCommandBuilder {
    pub(crate) fn new(subcommand_base: SubcommandBase, test_name: String) -> Self {
        RunTestCommandBuilder {
            subcommand_base,
            test_name,
            config_path: None,
        }
    }

    pub(crate) fn with_config<P: AsRef<Path>>(mut self, config_path: P) -> Self {
        self.config_path = Some(config_path.as_ref().to_path_buf());
        self
    }
}

impl SubcommandBuilder for RunTestCommandBuilder {
    fn assert(self) -> Assert {
        let mut cmd = self.subcommand_base.cmd();
        cmd.args(&["run", self.test_name.as_str()]);
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
