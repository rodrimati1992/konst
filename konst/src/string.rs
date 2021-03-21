//! `const fn` equivalents of `str` methods.

__declare_string_cmp_fns! {
    import_path = "konst",
    equality_fn = eq_str,
    ordering_fn = cmp_str,
    ordering_fn_inner = cmp_str_inner,
}

#[cfg(feature = "cmp")]
__declare_fns_with_docs! {
    (Option<&'a str>, (eq_option_str, cmp_option_str))

    docs(default)

    macro = __impl_option_cmp_fns!(
        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "cmp")))]
        for['a,]
        params(l, r)
        eq_comparison = crate::polymorphism::CmpWrapper(l).const_eq(r),
        cmp_comparison = crate::polymorphism::CmpWrapper(l).const_cmp(r),
        parameter_copyability = copy,
    ),
}
