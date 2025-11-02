
const fn to_bar<T>(foo: [&T; 2]) -> impl Sized {
    konst::destructure_rec!{ [&bar, baz] = foo}
    std::mem::forget(bar);
    baz
}

const fn to_bar_2<T>(foo: [&mut T; 2]) -> impl Sized {
    konst::destructure_rec!{ [&mut bar, baz] = foo}
    std::mem::forget(bar);
    baz
}


fn main(){}