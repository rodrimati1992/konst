This is the changelog, summarising changes in each version(some minor changes may be ommited).

# 0.2

### 0.2.19

Addeded `konst::iter` module, with const encodings of `Iterator` API:
- `konst::iter::collect_const` (macro)
- `konst::iter::eval` (macro)
- `konst::iter::for_each` (macro)
- `konst::iter::into_iter` (macro)
- `konst::iter::iterator_dsl` (documentation module)
- `konst::iter::IntoIterWrapper` (struct)
- `konst::iter::IsIntoIterKind` (struct)
- `konst::iter::IsIteratorKind` (struct)
- `konst::iter::IsNonIteratorKind` (struct)
- `konst::iter::IsStdKind` (struct)
- `konst::iter::IntoIterKind`(trait)


Add these items (conditional on the `"rust_1_64"` feature):
- `konst::string::split`
- `konst::string::rsplit`
- `konst::string::split_terminator`
- `konst::string::rsplit_terminator`
- `konst::string::split_once`
- `konst::string::rsplit_once`

Add these const iterators to the `string` module(conditional on the `"rust_1_64"` feature):
- `konst::string::Split`
- `konst::string::RSplit`
- `konst::string::SplitTerminator`
- `konst::string::RSplitTerminator`

Added these slice functions:
- `konst::slice::chunks`(requires the `"rust_1_64"` feature)
- `konst::slice::chunks_exact`(requires the `"rust_1_64"` feature)
- `konst::slice::iter`
- `konst::slice::iter_copied`(requires the `"rust_1_61"` feature)
- `konst::slice::windows` (requires the `"rust_1_64"` feature)

Added these slice iterator types:
- `konst::slice::Chunks` (requires the `"rust_1_64"` feature)
- `konst::slice::ChunksRev` (requires the `"rust_1_64"` feature)
- `konst::slice::ChunksExact` (requires the `"rust_1_64"` feature)
- `konst::slice::ChunksExactRev` (requires the `"rust_1_64"` feature)
- `konst::slice::Iter`
- `konst::slice::IterRev`
- `konst::slice::IterCopied` (requires the `"rust_1_61"` feature)
- `konst::slice::IterCopiedRev` (requires the `"rust_1_61"` feature)
- `konst::slice::Windows`
- `konst::slice::WindowsRev`

Added range iterator types:
- `konst::range::RangeIter`
- `konst::range::RangeIterRev`
- `konst::range::RangeInclusiveIter`
- `konst::range::RangeInclusiveIterRev`
- `konst::range::RangeFromIter`

Added `konst::option::copied` function (requires the `"rust_1_61"` feature)


Added `"rust_1_51"` feature, made `"const_generics"` feature delegate to it.

Added `"rust_1_61"` feature

Added `"rust_latest_stable"` feature that enables the latest `"rust_*"` feature.

### 0.2.16

Added `"rust_1_64"` crate feature, to make slice indexing take constant time from Rust 1.64.0 onwards.

Changed `"constant_time_slice"` to enable `"rust_1_64"` feature.

### 0.2.14

Made `"deref_raw_in_fn"` feature only require Rust 1.56.0 by stopping the use of an (already stabilized) nightly feature flag.

### 0.2.13

Added these functions:
- `konst::manually_drop::as_inner` (requires the `"rust_1_56"` feature).
- `konst::manually_drop::as_inner_mut` (requires the `"mut_refs"` feature).

Added these `MaybeUninit` functions:
- `konst::maybe_uninit::array_assume_init` (requires the `"rust_1_56"` feature)
- `konst::maybe_uninit::as_mut_ptr` (requires the `"mut_refs"` feature)
- `konst::maybe_uninit::as_ptr`
- `konst::maybe_uninit::assume_init_mut` (requires the `"mut_refs"` feature)
- `konst::maybe_uninit::assume_init_ref` (requires the `"rust_1_56"` feature)
- `konst::maybe_uninit::assume_init` (requires the `"rust_1_56"` feature)
- `konst::maybe_uninit::uninit_array` (requires the `"rust_1_56"` feature)
- `konst::maybe_uninit::write` (requires the `"mut_refs"` feature)

Added these raw pointer/`NonNull` functins:
- `konst::ptr::as_mut` (requires the `"mut_refs"` feature)
- `konst::ptr::as_ref` (requires the `"rust_1_56"` feature)
- `konst::ptr::deref_mut` (requires the `"mut_refs"` feature)
- `konst::ptr::deref` (requires the `"rust_1_56"` feature)
- `konst::ptr::is_null` (requires the `"rust_1_56"` feature)
- `konst::ptr::non_null::as_mut` (requires the `"mut_refs"` feature)
- `konst::ptr::non_null::as_ref` (requires the `"rust_1_56"` feature)
- `konst::ptr::non_null::from_mut` (requires the `"mut_refs"` feature)
- `konst::ptr::non_null::from_ref`
- `konst::ptr::non_null::new` (requires the `"rust_1_56"` feature)


### 0.2.12

Added `konst::array::map`, which requires the `rust_1_56` feature.

### 0.2.10

Added `"rust_1_55"` and `"rust_1_56"` features.

Change: functions that required the `"deref_raw_in_fn"` feature can be used on Rust 1.56.0 (whenever that reaches stable) with the `"rust_1_56"` feature.

Added these slice functions
- `konst::slice::bytes_find_keep`
- `konst::slice::bytes_find_skip`
- `konst::slice::bytes_rfind_keep`
- `konst::slice::bytes_rfind_skip`
- `konst::slice::bytes_trim_end_matches`
- `konst::slice::bytes_trim_end`
- `konst::slice::bytes_trim_matches`
- `konst::slice::bytes_trim_start_matches`
- `konst::slice::bytes_trim_start`
- `konst::slice::bytes_trim`
- `konst::slice::get_from_mut` (requires the `"mut_refs"` feature)
- `konst::slice::get_from`
- `konst::slice::get_mut` (requires the `"mut_refs"` feature)
- `konst::slice::get_range_mut` (requires the `"mut_refs"` feature)
- `konst::slice::get_range`
- `konst::slice::get_up_to_mut` (requires the `"mut_refs"` feature)
- `konst::slice::get_up_to`
- `konst::slice::get`



Added these str functions, all of which require the `"rust_1_55"` feature:
- `konst::string::from_utf8`
- `konst::string::get_from`
- `konst::string::get_range`
- `konst::string::get_up_to`
- `konst::string::split_at`
- `konst::string::str_from`
- `konst::string::str_range`
- `konst::string::strip_prefix`
- `konst::string::strip_suffix`
- `konst::string::str_up_to`
- `konst::string::trim`
- `konst::string::trim_start`
- `konst::string::trim_end`
- `konst::string::trim_matches`
- `konst::string::trim_start_matches`
- `konst::string::trim_end_matches`
- `konst::string::find_skip`
- `konst::string::find_keep`
- `konst::string::rfind_skip`
- `konst::string::rfind_keep`


Renamed these functions, reexported with old name, and deprecated the reexport:
- `string::str_starts_with`: renamed to `string::starts_with`
- `string::str_ends_with`: renamed to `string::ends_with`
- `string::str_find`: renamed to `string::find`
- `string::str_contains`: renamed to `string::contains`
- `string::str_rfind`: renamed to `string::rfind`
- `string::str_rcontains`: renamed to `string::rcontains`


Added the `konst::string::from_utf8` macro.

Added `konst::string::Utf8Error` error struct.


### 0.2.9

Fixed internal soundness bug in where unions weren't `#[repr(C))]`.

### 0.2.7

Added the  `"mut_refs"`, `"nightly_mut_refs"` features for const fns that take mutable references.

Added these mutable slice functions:
- `konst::slice::first_mut`
- `konst::slice::last_mut`
- `konst::slice::slice_from_mut`
- `konst::slice::slice_range_mut`
- `konst::slice::slice_up_to_mut`
- `konst::slice::split_at_mut`
- `konst::slice::split_first_mut`
- `konst::slice::split_last_mut`
- `konst::slice::try_into_array_mut`


### 0.2.5

Added `ParseFor` trait and `parse_with` macro, for type-dispatched parsing.

Added `StdParser` struct as the parser for std types.

Broadened valid syntax for try_rebind/rebind_if_ok

Added trailing comma support to every macros, where applicable


### 0.2.4

Fixed handling of length argument of `slice::try_into_array` macro,
when the "const_generics" feature is enabled,
it would previously require non-trivial expressions to be wrapped in braces.

### 0.2.3

Added the `slice::try_into_array` macro and nightly function to try to convert `&[T]` to `&[T; N]`

Defined the `slice::TryIntoArrayError` as the error type for `slice::try_into_array`

Added `slice::{first, last, split_first, split_last}` functions, const equivalents of the slice methods.

Added these (generic) constants, which can be used in `[CONST; LEN]` even for non-`Copy` types:

- `option::NONE`: alias for `None`
- `alloc_type::COW_STR_NEW`: An empty `Cow<'_, str<`
- `alloc_type::COW_SLICE_NEW`: An empty `Cow<'_, [T]>`
- `alloc_type::STRING_NEW`: An empty `String`
- `alloc_type::VEC_NEW}`: An empty `Vec<T>`
- `maybe_uninit::UNINIT`: An uninit `MaybeUninit<T>`
- `maybe_uninit::UNINIT_ARRAY`: An uninit `[MaybeUninit<T>; LEN]`

Added `"deref_raw_in_fn"` feature to enable `const fn`s that need to dereference raw pointers, requires the nightly compiler.

Added `"alloc"` feature to enable the `alloc_type` module.


### 0.2.1

Defined the `konst_macro_rules` crate 0.2.0, to reexport macros in `konst` submodules.

Added macro equivalents of `Option::{and_then, filter, map, ok_or, ok_or_else, or_else, unwrap_or, unwrap_or_else, flatten}` in the `konst::option` module.

Added macro equivalents of `Result::{unwrap_ctx, unwrap_or, unwrap_or_else, ok, err, map, map_err, and_then, or_else}` in the `konst::result` module.

Deprecated the `unwrap_opt_or`, and `unwrap_res_or` macros.

Defined `result::unwrap_err_or_else` macro for convenience.

Added `try_` and `try_opt` macros

Added `primitive::parse_*` functions, to parse all integer types and bool from `&str` or `&[u8]`.

Added `Parser::parse_` methods for the remaining integer types (`u8`, `u16`, `u32`, `i8`, `i16`, `i32`)

Added "parsing_no_proc" cargo feature, to enable all parser items except for those that require using the `konst_proc_macro` crate.

Fixed `eq_bytes` and `cmp_bytes` doc examples

Fixed the lack of indicators that the `rebind_if_ok` and `try_rebind` macros require the `"parsing_no_proc"` feature.

### 0.2.0

Breaking change: tightened the syntax of the `parse_any` macro, now it requires the last branch in the match-like syntax to be `_ => <expression>`, forbidding `_` patterns in other branches.

Added support for these  `Parser` methods in `parse_any`: 
- `find_skip` - `rfind_skip` - `trim_start_matches` - `trim_end_matches` 

Moved these functions from `slice::cmp` to `slice`, and reexported back in `slice::cmp`:
- `cmp_slice_u8`: renamed to `cmp_bytes`
- `eq_slice_u8`: renamed to `eq_bytes`
- `cmp_option_slice_u8`: renamed to `cmp_option_bytes`
- `eq_option_slice_u8`: renamed to `eq_option_bytes`
The above `*_bytes` functions do not require any cargo features to use.

Added comparison functions for `&[usize]`, `&[isize]`, `Option<&[usize]>`, `Option<&[isize]>` 


# 0.1

### 0.1.0

Defined the `konst_proc_macros` crate 0.1.0, for internal use by `konst`.

Defined equality and ordering comparison functions for:

- All the primitive types, including integers, bool, char, and `Option`s of them,
inside the `primitives::cmp` module.

- `Range`s and `RangeInclusive`s of all unsigned integer types, inside the `range::cmp` module.

- All the `NonZero*` types, and `Option<NonZero*>` types, inside the `nonzero::cmp` module.

- `&str` and `Option<&str>`, inside the `string` module.

- `Ordering`, `Option<Ordering>`, `PhantomData<T>`, `PhantomPinned`, inside the `other::cmp` module.

- Slices of integers, `&[bool]`, `&[char]`, `&[&str]`, `&[&[u8]]`, and `Option`s of them, inside the `slice::cmp` module.


Defined the polymorphism module, to do type-based overloading, currently limited to `const_eq` and `const_cmp` methods.


Defined these items in the `polymorphism` module:

- `CmpWrapper`: Wrapper type which implements many methods for std types, currently only `const_eq` and `const_cmp`.

- ConstCmpMarker: Trait for describing whether a type is from the standard library, and has `const_eq` and/or `const_cmp` methods.

- `IsAConstCmpMarker`: Helper type used to automatically coerce standard library types into CmpWrapper, and leaving non-std ones unwrapped.

- `IsArrayKind`, `IsNotStdKind`, `IsStdKind`: for describing the kind of type something is.


Declared these functions in the `slice` module:
`bytes_contain`, `bytes_end_with  `, `bytes_find  `, `bytes_rcontain  `, `bytes_rfind `, `bytes_start_with`, `bytes_strip_prefix  `, `bytes_strip_suffix  `, `slice_from  `, `slice_range `, `slice_up_to `, `split_at`.


Declared these functions in the `string` module:
`str_contains`, `str_ends_with`, `str_find`, `str_rcontains`, `str_rfind`, `str_starts_with`


Declared the `parsing` module with:

- `Parser`: A parser constructible from `&[u8]` or `&str`.

- `ParseError`: The error type returned by fallible `Parser` methods.

- `ErrorKind`: Enum for the kinds of parsing errors that happened, included inside `ParseError`

- `ParseDirection`: Enum for the direction a parser is currently parsing from, either the end or the start.

- `ParseValueResult`: Type alias for the `Result` types that `Parser` returns when parsing types.

- `ParserResult`: Type alias for `Result<'a, Parser, ParseError>`


Reexporting these types in the root module:
```rust
parsing::{Parser}
string::{cmp_str, eq_str, cmp_option_str, eq_option_str},
```

Declared these macros:

- `coerce_to_cmp`: For coercing values to types with `const_eq` or `const_cmp` methods.

- `const_cmp`, `const_cmp_for`, `const_eq`, `const_eq_for`: for comparing values

- `for_range`: for iterating over a range.

- `impl_cmp`: For implementing `ConstCmpMarker` for multiple types, and declaring `const_eq`/`const_cmp`.

- `konst`: For emulating inline const (`const{ }`).

- `parse_any`: For calling `Parser` methods with multiple alternative string literal patterns.

- `rebind_if_ok`: `if let Ok` which rebinds variables instead of shadowing them.

- `try_equal`: For returning early when an ordering isn't `Ordering::Equal`

- `try_rebind`: For returning early on `Err`, rebinding variables on `Ok`.

- `unwrap_ctx`: Unwraps a `Result`, calling `.panic()` on the error value on error.

- `unwrap_opt_or`: Const-equivalent of `Option::unwrap_or` and `Option::unwrap_or_else`.

- `unwrap_res_or`: Const-equivalent of `Result::unwrap_or` and `Result::unwrap_or_else`.



Defined these cargo features: 

- `"cmp"`(enabled by default): Enables all the comparison functions and macros.

- `"parsing"`(enabled by default): Enables the `parsing` module, macros, and compiles the `konst_proc_macros` crate.

- `"constant_time_slice"`: Improves the performance of certain slice functions from taking linear time to constant time. Requires Rust nightly.

- `"const_generics"`: Enables impls that use const generics, instead of being implemented for small arrays. Requires Rust 1.51.0 .





