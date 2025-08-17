use konst::slice;

const fn whatever0_return<T>() -> T {
    unimplemented!()
}

const _: () = {
    let arr = [0u8; 4];

    slice::fill_with!();
    slice::fill_with!(arr);
    slice::fill_with!(arr,);
    slice::fill_with!(arr,| wdwqd);
    slice::fill_with!(arr,||);
    slice::fill_with!(arr,|_|);
    slice::fill_with!(arr,{whatever0_return});
};



fn main() {}

