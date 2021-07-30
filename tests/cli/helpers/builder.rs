pub(crate) trait Builder: Default {
    type Target;

    fn build(self) -> Self::Target;
}
