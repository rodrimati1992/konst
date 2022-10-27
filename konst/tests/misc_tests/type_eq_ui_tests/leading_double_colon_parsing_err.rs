
// shadow core to test that leading `::` is parsed and used correctly
mod core {}

konst::polymorphism::type_eq_projection_fn!{
    pub(crate) fn project_res => ::core::result::Result<from T, E>
}

konst::polymorphism::type_eq_projection_fn!{
    pub(crate) fn project_opt => core::option::Option<from T>
}

fn main(){}