use konst::{result, slice};

const _: &[u8; 4] = result::unwrap!(slice::try_into_array(&[])) ;

const _: () = {
    let _: &mut [u8; 2] = result::unwrap!(slice::try_into_array_mut(&mut []));
};


fn main() {}