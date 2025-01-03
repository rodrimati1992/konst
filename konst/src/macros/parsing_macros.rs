
macro_rules! partial{
    (
        $new_macro:ident = $macro:ident ! ($($prefix:tt)*)
    ) => {
        __priv_partial!(
            ($)
            $new_macro = $macro ! ($($prefix)*)
        )
    };
}

macro_rules! __priv_partial {
    (
        ($_:tt)
        $new_macro:ident = $macro:ident ! ($($prefix:tt)*)
    ) => {
        macro_rules! $new_macro {
            ($_($_ args:tt)*) => {
                $macro!($($prefix)* $_($_ args)* )
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! try_parsing_ret_parser {
    ( $parser:ident, $parse_direction:ident $(,$ret:ident)*; $($code:tt)* ) => ({
        #![allow(unused_parens, unused_labels, unused_macros)]

        $parser.parse_direction = ParseDirection::$parse_direction;
        let copy = $parser.copy();

        let ($($ret),*) = 'ret: {
            partial!{throw = throw_out!(copy, $parse_direction,)}

            #[allow(unreachable_code)]
            {
                $($code)*
            }
        };

        enable_if_start!{$parse_direction,
            $parser.start_offset += (copy.str.len() - $parser.str.len()) as u32;
        }

        Ok(($($ret,)* $parser))
    })
}

macro_rules! try_parsing {
    ( $parser:ident, $parse_direction:ident $(,$ret:ident)*; $($code:tt)* ) => ({
        #![allow(unused_parens, unused_labels, unused_macros)]

        $parser.parse_direction = ParseDirection::$parse_direction;
        let copy = $parser.copy();

        let ($($ret),*) = 'ret: {
            partial!{throw = throw_out!(copy, $parse_direction,)}

            #[allow(unreachable_code)]
            {
                $($code)*
            }
        };

        enable_if_start!{$parse_direction,
            $parser.start_offset += (copy.str.len() - $parser.str.len()) as u32;
        }

        Ok(($($ret),*))
    })
}

macro_rules! parsing {
    ( $parser:ident, $parse_direction:ident $(,$ret:ident)*; $($code:tt)* ) => ({
        #![allow(unused_parens, unused_labels, unused_macros)]

        $parser.parse_direction = ParseDirection::$parse_direction;
        enable_if_start!{$parse_direction, let copy = $parser.copy(); }

        let ($($ret),*) = { $($code)* };

        enable_if_start!{$parse_direction,
            $parser.start_offset += (copy.str.len() - $parser.str.len()) as u32;
        }

        ($($ret),*)
    })
}

macro_rules! throw_out {
    ($copy:ident, $parse_direction:ident, $kind:expr) => {
        return Err(crate::parsing::ParseError::new($copy, $kind))
    };
    ($copy:ident, $parse_direction:ident, $kind:expr, map_err = $func:ident) => {
        return Err($func(crate::parsing::ParseError::new($copy, $kind)))
    };
}

macro_rules! enable_if_start{
    (FromEnd, $($tokens:tt)*) => { };
    (FromStart, $($tokens:tt)*) => {
        $($tokens)*
    };
    (FromBoth, $($tokens:tt)*) => {
        $($tokens)*
    };
}
