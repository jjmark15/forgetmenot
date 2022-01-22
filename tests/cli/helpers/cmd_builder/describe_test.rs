use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;
use derive_new::new;

use crate::helpers::cmd_builder::subcommand::{SubcommandBase, SubcommandBuilder};

#[derive(new)]
pub(crate) struct DescribeTestCommandBuilder {
    test_name: String,
    #[new(default)]
    config_path: Option<PathBuf>,
    subcommand_base: SubcommandBase,
}

impl DescribeTestCommandBuilder {
    pub(crate) fn with_config<P: AsRef<Path>>(mut self, config_path: P) -> Self {
        self.config_path = Some(config_path.as_ref().to_path_buf());
        self
    }
}

impl SubcommandBuilder for DescribeTestCommandBuilder {
    fn assert(self) -> Assert {
        let mut cmd = self.subcommand_base.cmd();
        cmd.args(&["describe", self.test_name.as_str()]);
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
