
// shadow core to test that leading `::` is parsed and used correctly
mod core {}

konst::polymorphism::type_eq_projection_fn!{
    pub(crate) fn project_res(T) -> ::core::result::Result<T, E>
}

konst::polymorphism::type_eq_projection_fn!{
    pub(crate) fn project_opt(T) -> core::option::Option<T>
}

fn main(){}