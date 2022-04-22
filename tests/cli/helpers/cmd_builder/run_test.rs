use std::path::{Path, PathBuf};

use assert_cmd::assert::Assert;
use derive_new::new;

use crate::helpers::cmd_builder::subcommand::{SubcommandBase, SubcommandBuilder};

#[derive(new)]
pub(crate) struct RunTestCommandBuilder {
    subcommand_base: SubcommandBase,
    test_name: String,
    #[new(default)]
    config_path: Option<PathBuf>,
    #[new(default)]
    env_vars: Vec<(String, String)>,
}

impl RunTestCommandBuilder {
    pub(crate) fn with_config<P: AsRef<Path>>(mut self, config_path: P) -> Self {
        self.config_path = Some(config_path.as_ref().to_path_buf());
        self
    }

    pub(crate) fn with_env_var<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> Self {
        self.env_vars
            .push((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self
    }
}

impl SubcommandBuilder for RunTestCommandBuilder {
    fn assert(self) -> Assert {
        let mut cmd = self.subcommand_base.cmd();

        cmd.envs(self.env_vars);
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
