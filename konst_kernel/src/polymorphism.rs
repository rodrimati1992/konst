pub mod kinds {

    /// Marker for user-defined types that can be converted into const iterators
    pub enum IsIntoIterKind {}

    /// Marker for const iterators
    pub enum IsIteratorKind {}

    /// Marker for references.
    pub enum IsRefKind {}

    /// Marker for non-standard library types.
    pub enum IsNotStdKind {}

    /// Marker for standard library types.
    pub enum IsStdKind {}
}
