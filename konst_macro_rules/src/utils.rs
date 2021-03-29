pub union Dereference<'a, T> {
    pub ptr: *const T,
    pub reff: &'a T,
}
