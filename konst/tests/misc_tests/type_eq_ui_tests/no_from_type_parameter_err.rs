use konst::polymorphism::type_eq_projection_fn;

type_eq_projection_fn!{
    ///
    fn project_option() -> Option<T>
}

type_eq_projection_fn!{
    ///
    fn project_option(T) -> Option<U>
}

type_eq_projection_fn!{
    ///
    fn project_ref(T) -> Ref<'a, U>
}
type Ref<'a, T> = &'a T;


fn main(){}