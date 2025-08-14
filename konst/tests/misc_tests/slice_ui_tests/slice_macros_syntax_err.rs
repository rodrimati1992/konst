use konst::slice;

const _: () = {
    let arr = [0u8; 4];

    slice::fill_with!();
    slice::fill_with!(arr);
    slice::fill_with!(arr,);
    slice::fill_with!(arr,| wdwqd);
    slice::fill_with!(arr,||);
    slice::fill_with!(arr,|_|);
};



fn main() {}

