struct Tuple();

impl Drop for Tuple {
    fn drop(&mut self) {}
}

const fn to_bar(foo: Tuple)  {
    konst::destructure_rec!{Tuple() = foo}
}

fn main(){}