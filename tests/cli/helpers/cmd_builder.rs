use assert_cmd::assert::Assert;
use assert_cmd::Command;

pub(crate) struct CliCommandBuilder {
    cmd: Command,
    subcommand: Option<CliSubcommand>,
}

impl CliCommandBuilder {
    pub(crate) fn new() -> Self {
        CliCommandBuilder {
            cmd: Command::cargo_bin("forgetmenot").unwrap(),
            subcommand: None,
        }
    }

    pub(crate) fn version(mut self) -> Self {
        self.select_subcommand(CliSubcommand::Version);
        self.cmd.arg("-V");
        self
    }

    fn select_subcommand(&mut self, subcommand: CliSubcommand) {
        self.subcommand = Some(subcommand);
    }

    pub(crate) fn assert(mut self) -> Assert {
        self.cmd.assert()
    }
}

enum CliSubcommand {
    Version,
}
