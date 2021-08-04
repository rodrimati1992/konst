macro_rules! impl_bytes_function {
    (
        strip_prefix;
        left = $left:expr;
        right = $right:expr;
    ) => {
        if $left.len() < $right.len() {
            return None;
        }

        loop {
            match ($left, $right) {
                ([lb, rem_slice @ ..], [rb, rem_matched @ ..]) => {
                    $left = rem_slice;
                    $right = rem_matched;

                    if *lb != *rb {
                        return None;
                    }
                }
                (rem, _) => break Some(rem),
            }
        }
    };
    (
        strip_suffix;
        left = $left:expr;
        right = $right:expr;
    ) => {
        if $left.len() < $right.len() {
            return None;
        }

        loop {
            match ($left, $right) {
                ([rem_slice @ .., lb], [rem_matched @ .., rb]) => {
                    $left = rem_slice;
                    $right = rem_matched;

                    if *lb != *rb {
                        return None;
                    }
                }
                (rem, _) => break Some(rem),
            }
        }
    };
}
