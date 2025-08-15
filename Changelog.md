This is the changelog, summarising changes in each version(some minor changes may be ommited).

# 0.4

### 0.4.0

Removed these items because there is an equivalent built-in replacement:
- `konst::alloc_type::COW_STR_NEW`: replaced by `const { Cow::Borrowed("") }`
- `konst::alloc_type::STRING_NEW`: replaced by `const { String::new() }`
- `konst::alloc_type::VEC_NEW`: replaced by `const { Vec::new() }`
- `konst::chr::from_u32_unchecked`
- `konst::chr::from_u32`
- `konst::ffi::cstr`: `CStr` constructors are const, unwrappable with `konst::result::unwrap`
- `konst::konst` macro: replaced by `const {  }` blocks
- `konst::maybe_uninit::as_mut_ptr`
- `konst::maybe_uninit::as_ptr`
- `konst::maybe_uninit::assume_init_ref`
- `konst::maybe_uninit::assume_init`
- `konst::maybe_uninit::UNINIT`: replaced by `const { MaybeUninit::uninit() }`
- `konst::option::copied`
- `konst::option::flatten`
- `konst::option::NONE`: replaced by `const { None }`
- `konst::option::unwrap`
- `konst::primitive::parse_i*` functions: replaced by integer `from_str_radix` methods, unwrappable with `konst::result::unwrap`
- `konst::primitive::parse_u*` functions: replaced by integer `from_str_radix` methods, unwrappable with `konst::result::unwrap`
- `konst::ptr::as_mut`: replaced by `&mut *pointer`
- `konst::ptr::as_ref`: replaced by `&*pointer`
- `konst::ptr::nonnull::as_mut`
- `konst::ptr::nonnull::as_ref`
- `konst::slice::as_chunks`
- `konst::slice::as_rchunks`
- `konst::slice::bytes_trim_end`
- `konst::slice::bytes_trim_start`
- `konst::slice::bytes_trim`
- `konst::slice::first_mut`
- `konst::slice::last_mut`
- `konst::slice::split_first_mut`
- `konst::slice::split_last_mut`
- `konst::string::from_utf8`
- `konst::string::is_char_boundary`
- `string::trim_end`: replaced by `str::trim_ascii_end`
- `string::trim_start`: replaced by `str::trim_ascii_start`
- `string::trim`: replaced by `str::trim_ascii`

Removed these items because their unstable std equivalents were removed:
- `konst::slice::array_chunks`
- `konst::slice::array_chunks_mut`
- `konst::slice::ArrayChunks`
- `konst::slice::ArrayChunksRev`
- `konst::slice::ArrayChunksMut`
- `konst::slice::ArrayChunksMutRev`


Removed these functions due to having an unsound impl:
- `konst::ptr::is_null`
- `konst::ptr::nonnull::new`

Removed these items because they are not needed anymore:
- `konst::rebind_if_ok` macro
- `konst::try_rebind` macro
- `konst::polymorphism::type_eq_projection_fn` macro: replaced by `typewit::TypeFn`+`TypeEq::project`
- `konst::unwrap_ctx` macro: superceeded by `konst::result::unwrap`
- `panic` methods: superceeded by `const_panic::PanicFmt` impls
- `konst::parsing::ParserResult`: using the type it aliases instead of the alias
- `konst::cmp::ConstCmpUnref`: replaced by `ConstCmp::This` associated type

Constrained macros that take closures from taking functions through `:expr` to `:path`.

Added these macros at the root module:
- `if_let_Some`
- `while_let_Some`


Replaced old `konst::array::{map, from_fn}` macros with `konst::array::{map_, from_fn_}` (renaming them in the process).

Change: Made array items conditional on `"iter"` feature:
- `konst::array::ArrayBuilder`
- `konst::array::from_fn`
- `konst::array::IntoIter`
- `konst::array::IntoIterRev`
- `konst::array::map`

Renamed `ArrayConsumer` into `IntoIter`, impld `ConstIterator` for it, and changed its `next*` methods to return `Option<T>` instead of `Option<ManuallyDrop<T>>` (to be consistent with `ConstIntoIter` API).

Added `konst::array::IntoIterRev` iterator.


Added these items for `ConstCmp` overhaul;
- `konst::cmp::ConstCmp::This` associated type
- `konst::cmp::ConstCmpKind` trait
- `konst::cmp::CoerceTo` type alias

Removed `IsRefKind` reexport from `konst::cmp` and made it not a valid `ConstCmp::Kind`

Merged all inherent `coerce` methods of `IsAConstCmp` into one generic method.

Replaced `CmpWrapper::slice` constructor with `from_ref`

Changed how `CmpWrapper` wraps arrays, now it doesn't coerce arrays to slices.

Changed how `CmpWrapper` wraps unsized types, now instead of having impls for `CmpWrapper<&[T]>` and `CmpWrapper<&str>`, there's impls for `CmpWrapper<[T]>` and `CmpWrapper<str>`.

Changed how CmpWrapper wraps `Sized` std types, now instead of copying the value,
it uses its `from_ref` constructor to convert `&T` to `&CmpWrapper<T>`.

Moved these macros to `konst::cmp` module: 
- `coerce_to_cmp`
- `const_cmp`
- `const_cmp_for`
- `const_eq`
- `const_eq_for`
- `impl_cmp`
- `max`
- `max_by`
- `max_by_key`
- `min`
- `min_by`
- `min_by_key`
- `try_equal`

Changed `konst::cmp::{const_cmp_for, const_eq_for}` to borrow their slice argument

Added these comparison macros:
- `konst::cmp::const_ge`
- `konst::cmp::const_ge_for`
- `konst::cmp::const_gt`
- `konst::cmp::const_gt_for`
- `konst::cmp::const_le`
- `konst::cmp::const_le_for`
- `konst::cmp::const_lt`
- `konst::cmp::const_lt_for`
- `konst::cmp::const_ne`
- `konst::cmp::const_ne_for`

Fixed how `const_cmp_for` macro and `const_cmp` methods compared slices for ordering, they used to consider all slices less than all longer slices.



Turned iterators from taking `Self` and returning `Option<(Self::Item, Self)>` into taking `&mut Self` and returning `Option<Self::Item>`.

Removed support for lifetime promotion in return value of iterator methods in iterator macros

Added `konst::iter::ConstIntoIter::ITEMS_NEED_DROP` associated constant

Added `konst::iter::ConstIntoIterKind` trait

Added `konst::iter::CoerceTo` type alias

Added `ConstIntoIter` impl for `&mut impl ConstIntoIter<Kind = IsIteratorKind>`, that is for mutable references to iterators.

Added `ConstIntoIterKind` bound to `ConstIntoIter::Kind`.

Turned `IntoIterWrapper::coerce` methods into free function at `konst::iter::coerce`

Added `konst::iter::{repeat_n, RepeatN}` iterator constructor and iterator struct.

Removed `rposition` method from iterator macros, since it returned the distance from the end, not the start (bugfix).

Changed drop behavior for iterators in `iter::eval`, now all iterators are forgotten, and those with `ConstIntoIter::ITEMS_NEED_DROP == true` are fully consumed after evaluating the methods passed to the macro.

Changed `IntoIterWrapper` to only be constructible by `konst::iter::coerce`, and made its fields private.

Changed `rev` method of iterator macros so that it can't be used if `enumerate`/`take`/`take_while`/`skip`/`skip_while`/`zip` were used before it, because the iterator without this restriction would produce different results than `std`.

Added these methods to `iter::eval` macro (conditional on "cmp" feature):
- `cmp`
- `eq`
- `ge`
- `gt`
- `is_sorted_by_key`
- `is_sorted_by`
- `is_sorted`
- `le`
- `lt`
- `max_by_key`
- `max_by`
- `max`
- `min_by_key`
- `min_by`
- `min`
- `ne`


Changed all of the `konst::parsing::Parser` methods to use `&mut self` instead of taking and returning `Parser` by value, changing the signature in this way:
- for `(self) -> Self` methods: changed into `(&mut self) -> &mut Self`
- for `(self, ..) -> Result<(T, Self), E>` methods: changed into `(&mut self, ..) -> Result<T, E>`.
- for `(self, ..) -> Result<Self, E>` methods: changed into `(&mut self, ..) -> Result<&mut Self, E>`.

Removed `Copy` impl for `Parser`, added `copy` method for explicit copying.

Renamed `Parser::into_error` to `to_error` and changed it from taking `self` to taking `&self`

Renamed `Parser::into_other_error` into `to_other_error`, and from taking `self` to `&self`.

Changed `ParseError::new` to take `Parser` by reference.

Changed `ParseError::other_error` to take `Parser` by reference.

Changed signature of `ParseError::extra_message` from returning &str to &'static str (the former ties the lifetime parameter of `&str` to the `ParseError` itself)

Moved these macros to `konst::parsing`:
- `parse_with`: and renamed to `parse_type`
- `parser_method`

Changed the `parse_type` macro to auto-borrow `konst::parsing::Parser`


Added `const_panic::PanicFmt` impls (used by `konst::result::unwrap`) for:
- `konst::parsing::ErrorKind`
- `konst::parsing::ParseError`
- `konst::parsing::ParseDirection`
- `konst::primiive::ParseBoolError`
- `konst::slice::TryIntoArrayError`


Changed `Option` macros to require `Option` by value when the equivalent methods takes `Option` by value (the macros used to allow passing `Option` by reference)

Added const equivalents of `Option`-related items:
- `konst::option::get_or_insert_with` (macro)
- `konst::option::get_or_insert` (macro)
- `konst::option::insert` (macro)
- `konst::option::into_iter` (function, iterator constructor)
- `konst::option::IntoIter` (struct, iterator)
- `konst::option::IntoIterRev` (struct, iterator)
- `konst::option::is_none_or` (macro)
- `konst::option::is_some_and` (macro)
- `konst::option::iter_mut` (function, iterator constructor)
- `konst::option::iter` (function, iterator constructor)
- `konst::option::Iter` (struct, iterator)
- `konst::option::IterMut` (struct, iterator)
- `konst::option::IterMutRev` (struct, iterator)
- `konst::option::IterRev` (struct, iterator)
- `konst::option::unzip` (function)
- `konst::option::zip` (macro)

Added support for `Drop` types in some `Option` macros:
- `konst::option::and_then`
- `konst::option::map`
- `konst::option::ok_or_else`
- `konst::option::or_else`
- `konst::option::unwrap_or_else`


Added missing impls of `ConstCmp` for `Range` and `RangeInclusive` types over signed integers.

Added `konst::range::OneSidedRange` trait (currently only used by `konst::slice::{split_off, split_off_mut}`)


Added these items to `konst::slice`:
- `konst::slice::chunks_exact_mut` (function, iterator constructor)
- `konst::slice::chunks_mut` (function, iterator constructor)
- `konst::slice::ChunksExactMut` (struct, iterator)
- `konst::slice::ChunksExactMutRev` (struct, iterator)
- `konst::slice::ChunksMut` (struct, iterator)
- `konst::slice::ChunksMutRev` (struct, iterator)
- `konst::slice::fill_with` (macro)
- `konst::slice::fill` (function)
- `konst::slice::is_sorted_by_key` (macro)
- `konst::slice::is_sorted_by` (macro)
- `konst::slice::is_sorted` (macro)
- `konst::slice::iter_mut` (function, iterator constructor)
- `konst::slice::IterMut` (struct, iterator)
- `konst::slice::IterMutRev` (struct, iterator)
- `konst::slice::rchunks_exact_mut` (function, iterator constructor)
- `konst::slice::rchunks_mut` (function, iterator constructor)
- `konst::slice::RChunksExactMut` (struct, iterator)
- `konst::slice::RChunksExactMutRev` (struct, iterator)
- `konst::slice::RChunksMut` (struct, iterator)
- `konst::slice::RChunksMutRev` (struct, iterator)
- `konst::slice::split_off_first_mut` (function)
- `konst::slice::split_off_first` (function)
- `konst::slice::split_off_last_mut` (function)
- `konst::slice::split_off_last` (function)
- `konst::slice::split_off_mut` (function)
- `konst::slice::split_off` (function)


Reexported `const_panic::unwrap_ok` as `konst::result::unwrap`

Changed `Result` macros to require passing Result by value whenever the equivalent method does.


Changed `konst`'s edition to 2024 and Minimum Supported Rust Version to 1.89.0.

Added crate features:
- `"const_panic_derive"`: to enable `const_panic`'s `"derive"` feature.

Removed these crate features:
- `"rust_1_*"`
- `"*mut_refs"`

Upgraded `const_panic` dependency to 0.2.13, enabled its `"rust_1_88"` feature

Upgraded `typewit` dependency to 1.12, disabled its default feature, and enabled its `"rust_1_83"` feature.

# 0.3

### 0.3.16

Deprecated these functions:
- `konst::ptr::is_null`
- `konst::ptr::nonnull::new`

### 0.3.15

Soft-removed `"mut_refs"` and `"nightly_mut_refs"` feature (by hidding it in docs), changed to using `"rust_1_83"` feature to enable `&mut`-taking `const fn`s. 
(to avoid breakage, `"mut_refs"` still enables items that use `&mut` in const by enabling `"rust_latest_stable"`)

Changed `"rust_latest_stable"` feature to enable `"rust_1_83"` feature.

Added `konst::manually_drop::take` function

Added these structs to `konst::array`:
- `ArrayBuilder`
- `ArrayConsumer`

### 0.3.14

Fixed accidental inclusion of private macros in docs.

### 0.3.12

Changed `konst::array::from_fn` macro to allow unparenthesized type to the left of the `=>

Added these macros in `konst::array` (which require the `"rust_1_83"` feature):
- `map_`
- `from_fn_`

### 0.3.10

Added `"rust_1_83"` feature, which enables the `"mut_refs"` feature. 

Added `"destructure"` macro (requires the `"rust_1_83"` feature)

Fixed future compatibility warnings

### 0.3.9

Defined these macros `max`, `max_by`, `max_by_key`, `min`, `min_by`, and `min_by_key` macros.

Re-exported these macros in the `konst::cmp` module:
- `coerce_to_cmp`
- `const_cmp`
- `const_cmp_for`
- `const_eq`
- `const_eq_for`
- `impl_cmp`
- `max`
- `max_by`
- `max_by_key`
- `min`
- `min_by`
- `min_by_key`


Now the `fold` and `foldr` methods in `konst::iter::eval` can take closures with type-annotated parameters (e.g: `|accum: Bar, element: Qux| {`)



### 0.3.8

Removed most `Copy` restriction of these macros:
- `konst::array::from_fn`
- `konst::array::map`
- `konst::iter::collect_const`
The pattern of the closure passed to `konst::array::map` is still required not to move out of the input array.

Improved `konst::array::{from_fn, map}` macros by allowing its closure parameter to take more unparenthesized patterns.

Improved the `konst::iter::{collect_const, eval, for_each}` macros by allowing single-parameter closures passed to the macro to annotate the type of the parameter.

### 0.3.7

Fixed `ChunksExact::remainder`'s implementation, it used to return the entire uniterated-over slice, when it should only return the part of the slice that the iterator never yields.

Added these items to `konst::slice` module:
- `ArrayChunksRev` struct
- `ArrayChunks` struct
- `RChunksExactRev` struct
- `RChunksExact` struct
- `RChunksRev` struct
- `RChunks` struct
- `array_chunks` function
- `as_chunks` function
- `as_rchunks` function
- `rchunks_exact` function
- `rchunks` function


### 0.3.6

Changed `type_eq_projection_fn` macro to not use any `unsafe`, this may require fixing bounds in your code.

### 0.3.5

Added `typewit = "1.1"` dependency.

Removed `konst::docs` module.

Replaced `konst::polymorphism::{HasTypeWitness,MakeTypeWitness,TypeWitnessTypeArg,TypeEq}` from the same-named items in `typewit`

Added reexport of `typewit::{self, CallFn, simple_type_witness,TypeFn,type_fn}` in `konst::polymorphism`


### 0.3.4

Fixed `Parser::parse_direction`'s `const`-ness by making it a const fn.

### 0.3.0

Removed `konst::array::try_into_array` macro (it's superceeded by the function of the same name)

Removed  functions

Removed these functions because the std equivalent is stably const:
- `konst::maybe_uninit::{as_ptr, assume_init, assume_init_ref}`
- `konst::ptr::{deref, deref_mut}`
- `konst::slice::{first, last, split_first, split_last}`

Removed extra `from` parameter of:
- `konst::slice::{bytes_contain, bytes_rcontain, bytes_find,  bytes_rfind}`
- `konst::string::{find, contains, rfind, rcontains}`

Added these items in `konst::polymorphism`:
- `HasTypeWitness`(trait)
- `MakeTypeWitness`(trait)
- `TypeEq`(struct)
- `TypeWitnessTypeArg`(trait)
- `type_eq_projection_fn`(macro)

Added `konst::docs::type_witnesses` documentation module.

Added `konst::chr` module, with these items:
- `Utf8Encoded`
- `encode_utf8`
- `from_u32_unchecked`
- `from_u32`

Renamed `IntoIterKind` to `ConstIntoIter`.

Added `Item` and `IntoIter` assoc types to ConstIntoIter, and added checks in iterator macros that `Item` matches the item's type.

Moved comparison-related from `konst::polymorphism` to new `konst::cmp` module.

Renamed `ConstCmpMarker` to `ConstCmp`.

Renamed these marker types:
- `IsAConstCmpMarker` to `IsAConstCmp`
- `IsIntoIterKind` to `IsConstIntoIter`
- `IsNonIteratorKind` to `IsIntoIterKind` (not to be confused with the previous name for the trait `ConstIntoIter` trait)

Removed `konst::polymorphism::IsArrayKind`

Added `konst::polymorphism::IsRefKind`
    
Added `konst::cmp::ConstCmpUnref` traits.

Moved all `Is*Kind` field-less structs to `konst::polymorphism::kinds`, made them all 0-variant enums, and reexported the relevant items in `konst::cmp` and `konst::iter`.

Removed `ConstCmp::This` associated type, now it's derived with `ConstCmpUnref`.

Added `konst::slice::BytesPattern` trait, implemented for `char`, `[u8]`, `[u8; N]`, and `str`.

Made the second parameter of `konst::slice::bytes_*` functions take a generic `BytesPattern`.

Added `konst::string::Pattern` trait, implemented for `char` and `&str`

Made `string` functions take `P: Pattern<'s>` as the pattern argument.

Bumped minimum supported Rust versions to 1.65.0

Added `const_panic = "0.2"` depdendency to `konst`

Made all panicking code use `const_panic` to have the same error messages as at runtime (previously use array indexing hack to panic).

Added `konst::array::from_fn` macro.

Added `konst::iter::{repeat, Repeat}`.

Added iteration over ranges of non-usize integers and `char`.
    
Added `konst::iter::Step` trait.

Added `konst::ffi::cstr` module.

Reexporting `konst::iter::step::Step` in `konst::iter`

Added these `konst::string` items:
- `Chars`(struct)
- `RChars`(struct)
- `CharIndices`(struct)
- `RCharIndices`(struct)
- `char_indices`(function)
- `chars`(function)
- `from_iter`(macro)
- `is_char_boundary`(function)
- `str_concat`(macro)
- `str_join`(macro)

Rewrote `Parser` to only support parsing `&str`.

Added `Parser::{skip, skip_back}` methods

Removed these parser functions:
- `Parser::*_b`
- `Parser::*_u8` (except for `parse_u8`)
- `Parser::advance_*`
- `Parser::from_bytes`

Renamed these `Parser` functions:
- `from_str` to `new`.
- `bytes` to `remainder` (and takes `&str`)

Added these `Parser` methods:
- `into_other_error`
- `rsplit_terminator`
- `rsplit`
- `split_keep`
- `split_terminator`
- `split`
- `skip`
- `skip_back`
- `trim`
- `trim_matches`
- `with_start_offset`

Added `ParseError::other_error` function for constructing `Other` errors with custom error messages.

Added `FromBoth` variant to `konst::parser::ParseDirection`

Renamed `ParserFor` trait to `HasParser`

Added `SplitExhausted`,`DelimiterNotFound` variants to `konst::parsing::ErrorKind`

Removed `map_err` capability of `try_rebind` due to being untested and unused.

Removed `unwrap_opt_or` and `unwrap_res_or` macros (they are superceeded by other macros).

Made `array::map` and iterator-taking macros support specifying an explicit return type.

Added `assertc_eq`, `assertc_ne` macros.

Added type annotation support to `rebind_if_ok` macro (for the pattern as a whole).

Fixed support for `continue` and `break` inside of `konst::for_range`.

Changed `Parser` methods from taking `&str` patterns to taking any `P: Pattern<'p>`

Renamed `parse_any` macro to `parser_method`

Removed `konst::primitive::parse_*_b` functions

Renamed `"parsing"` feature to `"parsing_proc"` and `"parsing_no_proc"` feature to `"parsing"`

Added `"iter"` feature(enabled by default), feature-gating all iterator-related items.

Added `"debug"` feature for additional safety checks in `konst` unsafe code.

Made `konst` crate 2021 edition, `resolver="2"`.

Removed all `"rust_1_*"` features, and all old-version-dependent code.

Renamed `konst_macro_rules` to `konst_kernel` crate, which defines items for reexporting in `konst` (mostly needed for having macros in submodules).



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


Added these const fn (enabled by the `"rust_1_64"` feature):
- `konst::string::split`
- `konst::string::rsplit`
- `konst::string::split_terminator`
- `konst::string::rsplit_terminator`
- `konst::string::split_once`
- `konst::string::rsplit_once`

Added these const string iterators (enabled by the `"rust_1_64"` feature):
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
- `konst::slice::Windows` (requires the `"rust_1_64"` feature)
- `konst::slice::WindowsRev` (requires the `"rust_1_64"` feature)

Added range iterator types:
- `konst::range::RangeIter`
- `konst::range::RangeIterRev`
- `konst::range::RangeInclusiveIter`
- `konst::range::RangeInclusiveIterRev`
- `konst::range::RangeFromIter`

Added `konst::option::copied` function (requires the `"rust_1_61"` feature)


Added `"rust_1_51"` feature, made `"const_generics"` feature delegate to it.

Added `"rust_1_57"` feature, which enables the `"rust_1_56"` feature.

Added `"rust_1_61"` feature, which enables the `"rust_1_57"` feature.

Changed `"rust_1_64"` feature: now it enables `"rust_1_61"` feature.

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





