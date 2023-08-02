use crate::misc_tests::test_utils::assert_type;

use konst::polymorphism::{type_eq_projection_fn, TypeEq};

use ::core::{fmt::Debug, marker::PhantomData};

#[test]
fn test_when_all_generic_args_are_passed() {
    #[derive(Debug, PartialEq, Clone)]
    struct Ty<'a, 'b: 'a, T: 'a + Debug, const N: usize>(&'a &'b [T; N]);

    type_eq_projection_fn! {
        fn project(T) -> Ty<'a, 'b: 'a, T: 'b + (Debug), const N: usize>
    }

    fn inner<'a, 'b: 'a, T: 'a + Debug, const N: usize>(
        te: TypeEq<u8, T>,
        this: Ty<'a, 'b, u8, N>,
    ) -> Ty<'a, 'b, T, N> {
        assert_type::<_, TypeEq<Ty<'a, 'b, u8, N>, Ty<'a, 'b, T, N>>>(&project::<_, _, N>(te));
        assert_type::<_, TypeEq<Ty<'a, 'b, u8, 10>, Ty<'a, 'b, T, 10>>>(&project::<_, _, 10>(te));

        project(te).to_right(this)
    }

    assert_eq!(inner(TypeEq::NEW, Ty(&&[3, 5, 8])), Ty(&&[3, 5, 8]));
}

#[test]
fn test_when_all_kinds_of_bounds_are_passed() {
    #[derive(Debug, PartialEq, Clone)]
    struct Ty<'a, 'b: 'a + 'a, 'c, T: 'a + 'b + Debug + Clone>(
        [T; 3],
        PhantomData<(&'a (), &'b (), &'c ())>,
    );

    type_eq_projection_fn! {
        fn project_one(T) -> Ty<'a, 'b: 'a + 'a, 'c, T: 'a + 'b + (Debug + Clone)>
    }
    type_eq_projection_fn! {
        fn project_two(T) -> Ty<'a, 'b: 'a + 'a, 'c, T: ('a + 'b + Debug + Clone)>
    }

    fn inner<'a, 'b: 'a + 'a, 'c, T: 'a + 'b + Debug + Clone>(
        te: TypeEq<usize, T>,
        this: Ty<'a, 'b, 'c, usize>,
    ) -> Ty<'a, 'b, 'c, T> {
        assert_type::<_, TypeEq<Ty<'a, 'b, 'c, usize>, Ty<'a, 'b, 'c, T>>>(&project_one::<_, _>(
            te,
        ));
        assert_type::<_, TypeEq<Ty<'a, 'b, 'c, usize>, Ty<'a, 'b, 'c, T>>>(&project_two::<_, _>(
            te,
        ));

        project_one(te).to_right(this)
    }

    assert_eq!(
        inner(TypeEq::NEW, Ty([3, 5, 8], PhantomData)),
        Ty([3, 5, 8], PhantomData),
    );
}

#[test]
fn test_lifetime_bounds_in_parentheses() {
    #[derive(Debug, PartialEq, Clone)]
    struct Ty<'a, 'b, T: 'a + 'b, U: std::fmt::Debug>(&'a &'b T, U);

    type_eq_projection_fn! {
        fn project_one(T) -> Ty<'a, 'b, T: ('a + 'b), U>
        where
            U: std::fmt::Debug,
    }
    type_eq_projection_fn! {
        fn project_two(T) -> Ty<'a, 'b, T: ('a + 'b +), U>
        where
            u32:,
            U: std::fmt::Debug
    }

    fn inner<'a, 'b, T: 'a + 'b, U: std::fmt::Debug>(
        te: TypeEq<usize, T>,
        this: Ty<'a, 'b, usize, U>,
    ) -> Ty<'a, 'b, T, U> {
        assert_type::<_, TypeEq<Ty<'a, 'b, usize, U>, Ty<'a, 'b, T, U>>>(&project_one::<_, _, U>(
            te,
        ));
        assert_type::<_, TypeEq<Ty<'a, 'b, usize, U>, Ty<'a, 'b, T, U>>>(&project_two::<_, _, U>(
            te,
        ));

        project_two(te).to_right(this)
    }

    assert_eq!(inner(TypeEq::NEW, Ty(&&3, 13u8)), Ty(&&3, 13u8),);
}

mod opt_pub {
    konst::polymorphism::type_eq_projection_fn! {
        pub fn project_opt(T) -> Option<T>
    }
}

mod opt_pub_crate {
    // shadow core to test that leading `::` is parsed and used correctly
    mod core {}

    konst::polymorphism::type_eq_projection_fn! {
        pub(crate) fn project_opt(T) -> ::core::option::Option<T>
    }
}

#[test]
fn test_visibility() {
    for te in [
        opt_pub::project_opt(TypeEq::new::<u8>()),
        opt_pub_crate::project_opt(TypeEq::new::<u8>()),
    ] {
        assert_eq!(te.to_right(Some(Default::default())), Some(0));
    }
}
