struct Tuple();

impl Drop for Tuple {
    fn drop(&mut self) {}
}

const fn to_bar(foo: Tuple)  {
    konst::destructure_rec!{Tuple() = foo}
}

const fn to_bar_nested(foo: [Tuple; 2])  {
    konst::destructure_rec!{[Tuple(), forg] = foo}

    core::mem::forget(forg);
}

fn main(){}