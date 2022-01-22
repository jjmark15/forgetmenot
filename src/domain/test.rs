use crate::domain::command::TestCommand;

#[derive(derive_new::new, derive_getters::Getters)]
pub(crate) struct Test {
    name: String,
    command: TestCommand,
    description: Option<String>,
}
