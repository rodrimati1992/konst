macro_rules! assertc_eq_rets {
    ($ty:ty, $function:expr => $left:expr, $right:expr, $expected:expr) => {{
        use const_cmp::const_eq;

        let left: $ty = $left;
        let right: $ty = $right;

        assert_eq!(
            $function(left, right),
            $expected,
            "{{A}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            const_eq!(left, right),
            $expected,
            "{{B}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(left == right, $expected, "{{C}}\n{:?}\n{:?}", left, right);

        assert_eq!(
            $function(right, left),
            $expected,
            "{{D}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            const_eq!(right, left),
            $expected,
            "{{E}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(right == left, $expected, "{{F}}\n{:?}\n{:?}", left, right);

        assert_eq!(
            $function(left, left),
            true,
            "{{G}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            const_eq!(left, left),
            true,
            "{{H}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(left == left, true, "{{I}}\n{:?}\n{:?}", left, right);

        assert_eq!(
            $function(right, right),
            true,
            "{{J}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            const_eq!(right, right),
            true,
            "{{K}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(right == right, true, "{{L}}\n{:?}\n{:?}", left, right);
    }};
}

macro_rules! assertc_opt_eq_rets {
    (
        $ty:ty, $function:expr $(, $opt_function:expr)? =>
        $(($left:expr, $right:expr, $expected:expr))*
    ) => {
        let cases: Vec<($ty, $ty, _)> = vec![
            $( ($left, $right, $expected), )*
        ];

        for (left, right, expected) in cases {
            let mleft = || <$ty as Clone>::clone(&left);
            let mright = || <$ty as Clone>::clone(&right);

            assertc_eq_rets!{$ty, $function => mleft(), mright(), expected}

            $(
            #[cfg(feature = "option")]
            {
                let cases = vec![
                    (Some(mleft()), Some(mright()), expected),
                    (Some(mleft()), None, false),
                    (None, Some(mleft()), false),
                    (None, None, true),
                ];

                for (l, r, e) in cases {
                    assertc_eq_rets!{Option<$ty>, $opt_function => l, r, e }
                }
            }
            )?

        }
    };
}

macro_rules! assertc_cmp {
    ($ty:ty, $function:expr => $left:expr, $right:expr, $expected:expr) => {{
        use const_cmp::const_cmp;

        let left: $ty = $left;
        let right: $ty = $right;

        assert_eq!(
            const_cmp!(left, right),
            $expected,
            "{{A}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            $function(left, right),
            $expected,
            "{{B}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            left.cmp(&right),
            $expected,
            "{{C}}\n{:?}\n{:?}",
            left,
            right
        );

        assert_eq!(
            const_cmp!(left, left),
            Equal,
            "{{D}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            $function(left, left),
            Equal,
            "{{E}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            $function(right, right),
            Equal,
            "{{F}}\n{:?}\n{:?}",
            left,
            right
        );

        assert_eq!(
            const_cmp!(right, left),
            $expected.reverse(),
            "{{G}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            $function(right, left),
            $expected.reverse(),
            "{{H}}\n{:?}\n{:?}",
            left,
            right
        );
        assert_eq!(
            right.cmp(&left),
            $expected.reverse(),
            "{{I}}\n{:?}\n{:?}",
            left,
            right
        );
    }};
}

macro_rules! assertc_opt_cmp {
    (
        $ty:ty, $function:expr, $opt_function:expr =>
        $(($left:expr, $right:expr, $expected:expr))*
    ) => {
        let cases: Vec<($ty, $ty, _)> = vec![
            $( ($left, $right, $expected), )*
        ];

        for (left, right, expected) in cases {
            let mleft = || <$ty as Clone>::clone(&left);
            let mright = || <$ty as Clone>::clone(&right);

            assertc_cmp!{$ty, $function => mleft(), mright(), expected}

            #[cfg(feature = "option")]
            {
                let cases = vec![
                    (Some(mleft()), Some(mright()), expected),
                    (Some(mleft()), None, Greater),
                    (None, Some(mleft()), Less),
                    (None, None, Equal),
                ];

                for (l, r, e) in cases {
                    assertc_cmp!{Option<$ty>, $opt_function => l, r, e }
                }
            }
        }
    };
}
