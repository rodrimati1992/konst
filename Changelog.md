This is the changelog, summarising changes in each version(some minor changes may be ommited).

# 0.1

### 0.1.0

Defined the `konst_proc_macros` craet 0.1.0, for internal use by `konst`.

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





