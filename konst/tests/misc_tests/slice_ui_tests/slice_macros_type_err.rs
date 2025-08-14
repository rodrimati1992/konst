use konst::slice;

const fn mismatched_func() -> u16 {
    0
}

const _: () = {
    let arr = [0u8; 4];

    slice::fill_with!(arr,|| 0);
    slice::fill_with!(&mut arr,|| 0u16);
    slice::fill_with!(&mut arr, mismatched_func);
};



fn main() {}

