use konst::polymorphism::type_eq_projection_fn;

type_eq_projection_fn!{
    ///
    fn project_option => Option<T>
}

type_eq_projection_fn!{
    ///
    fn project_ref => Ref<'a, T>
}
type Ref<'a, T> = &'a T;


fn main(){}