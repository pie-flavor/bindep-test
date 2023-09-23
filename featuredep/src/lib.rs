pub fn feature_enabled() -> bool {
    cfg!(feature = "foo")
}
