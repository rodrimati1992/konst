use konst::drop_flavor::{self, NonDrop};

fn unwrap() {
    let _ = drop_flavor::unwrap::<NonDrop, u8>(0);
}

fn as_inner() {
    let _ = drop_flavor::as_inner::<NonDrop, _>(&());
}

fn as_inner_mut() {
    let _ = drop_flavor::as_inner_mut::<NonDrop, _>(&mut ());
}

fn wrap() {
}


fn main() {}