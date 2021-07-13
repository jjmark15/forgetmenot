#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct VcsVersion {
    value: String,
}

impl VcsVersion {
    pub(crate) fn new(value: String) -> Self {
        VcsVersion { value }
    }

    pub(crate) fn value(&self) -> &String {
        &self.value
    }
}
