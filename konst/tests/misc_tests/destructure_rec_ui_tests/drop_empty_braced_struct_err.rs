struct Braced {}

impl Drop for Braced {
    fn drop(&mut self) {}
}


const fn to_bar(foo: Braced) {
    konst::destructure_rec!{Braced{} = foo}
}

const fn to_bar_nested(foo: [Braced; 2]) {
    konst::destructure_rec!{[Braced{}, other] = foo}

    core::mem::forget(other);
}


fn main(){}