struct Braced {}

impl Drop for Braced {
    fn drop(&mut self) {}
}


const fn to_bar(foo: Braced) {
    konst::destructure!{Braced{} = foo}
}


fn main(){}