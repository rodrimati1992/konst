#![allow(unused_imports)]

mod opt_pub_self {
    konst::polymorphism::type_eq_projection_fn!{
        pub(self) fn project_opt => Option<from T>
    }
}

const _: () = {
    opt_pub_self::project_opt::<u8, u8>;
};


mod opt_pub_super {
    pub mod foo {
        konst::polymorphism::type_eq_projection_fn!{
            pub(super) fn project_opt => Option<from T>
        }
    }

    const _: () = {
        foo::project_opt::<u8, u8>;
    };
}

const _: () = {
    opt_pub_super::foo::project_opt::<u8, u8>;
};

fn main(){}