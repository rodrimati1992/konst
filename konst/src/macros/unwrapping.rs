/// For unwrapping `Option`s in const contexts
#[macro_export]
macro_rules! unwrap_opt {
    ($e:expr) => {
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => loop {
                ["Attemped to unwrap a None"][101];
            },
        }
    };
}

/// For unwrapping `Option`s in const contexts, with a default value when it's a `None`.
#[macro_export]
macro_rules! unwrap_opt_or {
    ($e:expr, |$(_)?| $v:expr) => {
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => $v,
        }
    };
    ($e:expr, $v:expr) => {{
        let value = $v;
        match $e {
            $crate::__::Some(x) => x,
            $crate::__::None => value,
        }
    }};
}

/// For unwrapping `Result`s in const contexts
#[macro_export]
macro_rules! unwrap_res {
    ($e:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => loop {
                ["Attemped to unwrap an Err variant"][101];
            },
        }
    };
}

/// For unwrapping `Result`s in const contexts,
/// with a default value when it's an error.
#[macro_export]
macro_rules! unwrap_res_or {
    ($e:expr, |$(_)?| $v:expr) => {
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => $v,
        }
    };
    ($e:expr, $v:expr) => {{
        let value = $v;
        match $e {
            $crate::__::Ok(x) => x,
            $crate::__::Err(_) => value,
        }
    }};
}
