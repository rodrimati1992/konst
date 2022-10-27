use konst::polymorphism::type_eq_projection_fn;

type_eq_projection_fn!{
    ///
    fn project_option => Option<from T>
    where
        T: core::fmt::Debug
}

fn main(){}