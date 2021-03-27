[![Rust](https://github.com/rodrimati1992/konst/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/konst/actions)
[![crates-io](https://img.shields.io/crates/v/konst.svg)](https://crates.io/crates/konst)
[![api-docs](https://docs.rs/konst/badge.svg)](https://docs.rs/konst/*)

Compile-time comparison, parsing, and const equivalents of std methods.

# Features

This crate provides:

- Compile-time parsing through the [`Parser`] type, and [`parse_any`] macro.

- Functions for comparing many standard library types,
with the [`const_eq`]/[`const_eq_for`]/[`const_cmp`]/[`const_cmp_for`] macros
for more conveniently calling them, powered by the [`polymorphism`] module.

- Const fn equivalents of other standard library functions and methods.

# Examples

### Parsing an enum

This example demonstrates how you can parse a simple enum from an environment variable,
at compile-time.

```rust
use konst::eq_str;
use konst::{unwrap_opt_or, unwrap_ctx};

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    const fn try_parse(input: &str) -> Result<Self, ParseDirectionError> {
        // As of Rust 1.51.0, string patterns don't work in const contexts
        match () {
            _ if eq_str(input, "forward") => Ok(Direction::Forward),
            _ if eq_str(input, "backward") => Ok(Direction::Backward),
            _ if eq_str(input, "left") => Ok(Direction::Left),
            _ if eq_str(input, "right") => Ok(Direction::Right),
            _ => Err(ParseDirectionError),
        }
    }
}

const CHOICE: &str = unwrap_opt_or!(option_env!("chosen-direction"), "forward");

const DIRECTION: Direction = unwrap_ctx!(Direction::try_parse(CHOICE));

fn main() {
    match DIRECTION {
        Direction::Forward => assert_eq!(CHOICE, "forward"),
        Direction::Backward => assert_eq!(CHOICE, "backward"),
        Direction::Left => assert_eq!(CHOICE, "left"),
        Direction::Right => assert_eq!(CHOICE, "right"),
    }
}

```


### Parsing integers

You can parse integers using the `parse_*` functions in [`primitive`],
returning a `None` if the string as a whole isn't a valid integer.

```rust
use konst::primitive::parse_i128;

const N_100: Option<i128> = parse_i128("100");
assert_eq!(N_100, Some(100));

const N_N3: Option<i128> = parse_i128("-3");
assert_eq!(N_N3, Some(-3));

const NONE: Option<i128> = parse_i128("-");
assert_eq!(NONE, None);

const PAIR: Option<i128> = parse_i128("1,2");
assert_eq!(PAIR, None);



```

For parsing an integer inside a larger string,
you can use [`Parser::parse_u128`] method and the other `parse_*` methods

```rust
use konst::{Parser, unwrap_ctx};

const PAIR: (i64, u128) = {;
    let parser = Parser::from_str("1365;6789");

    // Parsing "1365"
    let (l, parser) = unwrap_ctx!(parser.parse_i64());

    // Skipping the ";"
    let parser = unwrap_ctx!(parser.strip_prefix(";"));

    // Parsing "6789"
    let (r, parser) = unwrap_ctx!(parser.parse_u128());
    
    (l, r)
};
assert_eq!(PAIR.0, 1365);
assert_eq!(PAIR.1, 6789);

```



### Parsing a struct

This example demonstrates how you can use [`Parser`] to parse a struct at compile-time.

```rust
use konst::{
    parsing::{Parser, ParseValueResult},
    for_range, parse_any, try_rebind, unwrap_ctx,
};

const PARSED: Struct = {
    // You can also parse strings from environment variables, or from an `include_str!(....)`
    let input = "\
        1000,
        circle,
        red, blue, green, blue,
    ";
    
    unwrap_ctx!(parse_struct(Parser::from_str(input))).0
};

fn main(){
    assert_eq!(
        PARSED,
        Struct{
            amount: 1000,
            repeating: Shape::Circle,
            colors: [Color::Red, Color::Blue, Color::Green, Color::Blue],
        }
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Struct {
    pub amount: usize,
    pub repeating: Shape,
    pub colors: [Color; 4],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Square,
    Line,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub const fn parse_struct(mut parser: Parser<'_>) -> ParseValueResult<'_, Struct> {
    try_rebind!{(let amount, parser) = parser.trim_start().parse_usize()}
    try_rebind!{parser = parser.strip_prefix(",")}

    try_rebind!{(let repeating, parser) = parse_shape(parser.trim_start())}
    try_rebind!{parser = parser.strip_prefix(",")}

    try_rebind!{(let colors, parser) = parse_colors(parser.trim_start())}

    Ok((Struct{amount, repeating, colors}, parser))
}

pub const fn parse_shape(mut parser: Parser<'_>) -> ParseValueResult<'_, Shape> {
    let shape = parse_any!{parser, strip_prefix;
        "circle" => Shape::Circle,
        "square" => Shape::Square,
        "line" => Shape::Line,
        _ => return Err(parser.into_other_error())
    };
    Ok((shape, parser))
}

pub const fn parse_colors(mut parser: Parser<'_>) -> ParseValueResult<'_, [Color; 4]> {
    let mut colors = [Color::Red; 4];

    for_range!{i in 0..4 =>
        try_rebind!{(colors[i], parser) = parse_color(parser.trim_start())}
        try_rebind!{parser = parser.strip_prefix(",")}
    }

    Ok((colors, parser))
}

pub const fn parse_color(mut parser: Parser<'_>) -> ParseValueResult<'_, Color> {
    let color = parse_any!{parser, strip_prefix;
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        _ => return Err(parser.into_other_error())
    };
    Ok((color, parser))
}



```

# Cargo features

These are the features of these crates:

- `"cmp"`(enabled by default):
Enables all comparison functions and macros,
the string equality and ordering comparison functions don't require this feature.


- `"parsing"`(enabled by default):
Enables the `"parsing_no_proc"` feature, compiles the `konst_proc_macros` dependency, 
and enables the [`parse_any`] macro. 
You can use this feature instead of `"parsing_no_proc"` if the slightly longer
compile times aren't a problem.

- `"parsing_no_proc"`(enabled by default):
Enables the [`parsing`] module (for parsing from `&str` and `&[u8]`),
and the `primitive::parse_*` functions.



- `"constant_time_slice"`(disabled by default):<br>
Improves the performance of slice functions that split slices,
from taking linear time to taking constant time,
this requires using some nightly Rust features.
<br>Note that only functions which mention this feature in their documentation are affected.

- `"const_generics"` (disabled by default):
Changes impls for arrays to use const generics instead of only supporting small arrays.
This feature requires Rust 1.51.0.

# No-std support

`konst` is `#![no_std]`, it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`konst` requires Rust 1.46.0, because it uses looping an branching in const contexts.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with cargo features.

[`const_eq`]: https://docs.rs/konst/*/konst/macro.const_eq.html
[`const_eq_for`]: https://docs.rs/konst/*/konst/macro.const_eq_for.html
[`const_cmp`]: https://docs.rs/konst/*/konst/macro.const_cmp.html
[`const_cmp_for`]: https://docs.rs/konst/*/konst/macro.const_cmp_for.html
[`polymorphism`]: https://docs.rs/konst/*/konst/polymorphism/index.html
[`parsing`]: https://docs.rs/konst/*/konst/parsing/index.html
[`primitive`]: https://docs.rs/konst/*/konst/primitive/index.html
[`parse_any`]: macro.parse_any.html
[`Parser`]: https://docs.rs/konst/*/konst/parsing/struct.Parser.html
[`Parser::parse_u128`]: https://docs.rs/konst/*/konst/parsing/struct.Parser.html#method.parse_u128