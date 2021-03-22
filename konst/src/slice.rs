//! `const fn` equivalents of slice methods.

/// `const fn`s for comparing slices for equality and ordering.
#[cfg(feature = "cmp")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
pub mod cmp;

mod slice_const_methods;

pub use slice_const_methods::*;

__declare_slice_cmp_fns! {
    import_path = "konst",

    (
        ///
        ///  # Example
        ///
        ,
        /// ```rust
        /// use slice::slice::eq_bytes;
        ///
        /// const FOO: &[u8] = b"foo";
        /// const BAR: &[u8] = b"fooooo";
        /// const BAZ: &[u8] = b"bar";
        ///
        ///
        /// const FOO_EQ_FOO: bool = eq_str(FOO, FOO);
        /// assert!( FOO_EQ_FOO );
        ///
        /// const FOO_EQ_BAR: bool = eq_str(FOO, BAR);
        /// assert!( !FOO_EQ_BAR );
        ///
        /// const FOO_EQ_BAZ: bool = eq_str(FOO, BAZ);
        /// assert!( !FOO_EQ_BAZ );
        ///
        /// ```
        ///
        ,
        /// ```rust
        /// use slice::slice::cmp_bytes;
        ///
        /// const FOO: &[u8] = b"foo";
        /// const BAR: &[u8] = b"fooooo";
        /// const BAZ: &[u8] = b"bar";
        ///
        ///
        /// const FOO_CMP_FOO: Ordering = cmp_str(FOO, FOO);
        /// assert_eq!(FOO_CMP_FOO, Ordering::Equal);
        ///
        /// const FOO_CMP_BAR: Ordering = cmp_str(FOO, BAR);
        /// assert_eq!(FOO_CMP_BAR, Ordering::Less);
        ///
        /// const FOO_CMP_BAZ: Ordering = cmp_str(FOO, BAZ);
        /// assert_eq!(FOO_CMP_BAZ, Ordering::Greater);
        ///
        /// ```
        ///
        ,
        u8,
        eq_bytes,
        cmp_bytes,
    )
}

__declare_fns_with_docs! {
    (Option<&'a [u8]>, (eq_option_bytes, cmp_option_bytes))

    docs(default)

    macro = __impl_option_cmp_fns!(
        for['a,]
        params(l, r)
        eq_comparison = eq_bytes(l, r),
        cmp_comparison = cmp_bytes(l, r),
        parameter_copyability = copy,
    ),
}
