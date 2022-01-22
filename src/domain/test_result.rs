#[derive(derive_new::new, derive_getters::Getters)]
pub(crate) struct TestResult {
    exit_code: i32,
}
