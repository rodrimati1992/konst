
use konst::polymorphism::type_eq_projection_fn;

type_eq_projection_fn!{
    ///
    fn project_option(T) -> Option<T>
    where 
        u32: std::fmt::Debug,
    {}

    fn foo() {}
}



fn main(){}
