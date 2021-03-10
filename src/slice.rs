__declare_cmp_fns! {
    import_path = "const_cmp",

    (,,, u8, slice_eq_u8, slice_cmp_u8,)
    (,,, u16, slice_eq_u16, slice_cmp_u16,)
    (,,, u32, slice_eq_u32, slice_cmp_u32,)
    (,,, u64, slice_eq_u64, slice_cmp_u64,)
    (,,, u128, slice_eq_u128, slice_cmp_u128,)

    (,,, i8, slice_eq_i8, slice_cmp_i8,)
    (,,, i16, slice_eq_i16, slice_cmp_i16,)
    (,,, i32, slice_eq_i32, slice_cmp_i32,)
    (,,, i64, slice_eq_i64, slice_cmp_i64,)
    (,,, i128, slice_eq_i128, slice_cmp_i128,)

    (,,, bool, slice_eq_bool, slice_cmp_bool,)
    (,,, char, slice_eq_char, slice_cmp_char,)
}
