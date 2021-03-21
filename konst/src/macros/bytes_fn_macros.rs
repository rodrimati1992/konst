macro_rules! impl_bytes_function {
    (
        strip_prefix;
        left = $left:expr;
        right = $right:expr;
        on_error = $on_error:expr;
    ) => {
        if $left.len() < $right.len() {
            $on_error;
        }

        while let ([lb, rem_slice @ ..], [rb, rem_matched @ ..]) = ($left, $right) {
            $left = rem_slice;
            $right = rem_matched;

            if *lb != *rb {
                $on_error;
            }
        }
    };
    (
        strip_suffix;
        left = $left:expr;
        right = $right:expr;
        on_error = $on_error:expr;
    ) => {
        if $left.len() < $right.len() {
            $on_error;
        }

        while let ([rem_slice @ .., lb], [rem_matched @ .., rb]) = ($left, $right) {
            $left = rem_slice;
            $right = rem_matched;

            if *lb != *rb {
                $on_error;
            }
        }
    };
}
