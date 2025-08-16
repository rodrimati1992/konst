use konst::drop_flavor;

use std::mem::ManuallyDrop as MD;

fn unwrap() {
    let _ = drop_flavor::unwrap(0);
}

fn as_inner() {
    let _ = drop_flavor::as_inner(&());
}

fn as_inner_mut() {
    let _ = drop_flavor::as_inner_mut(&mut ());
}

fn wrap() {
    let _ = drop_flavor::wrap(0);
    let _: &str = drop_flavor::wrap(0u8);
    
    // should work
    let _: MD<u8> = drop_flavor::wrap(0);
    let _: u8 = drop_flavor::wrap(0);
}


fn main() {}