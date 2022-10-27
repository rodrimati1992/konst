
use konst::polymorphism::type_eq_projection_fn;

type_eq_projection_fn!{
    ///
    fn project_option => Option<from T>
    where 
        u32: std::fmt::Debug,
    {}

    fn foo() {}
}



fn main(){}
