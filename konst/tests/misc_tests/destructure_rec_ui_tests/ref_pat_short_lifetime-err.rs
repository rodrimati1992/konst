
const fn to_bar<'a>(foo: (u32, u32)) -> (&'a u32, &'a mut u32) {
    konst::destructure_rec!{(ref bar, ref mut baz) = foo}
    (bar, baz)
}


fn main(){}
