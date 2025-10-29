use konst::array::ArrayBuilder;


const _: () = {
    let mut this = ArrayBuilder::of_copy::<2>();
    this.push(3u8);
    this.push(5u8);
    this.push(8u8);
};

const _: () = ArrayBuilder::of_copy::<2>().extend_from_array([3u8, 5, 8]);

const _: () = ArrayBuilder::of_copy::<2>().extend_from_slice(&[3u8, 5, 8]);

const _: [u8; 2] = {
    let mut this = ArrayBuilder::of_copy::<2>();
    this.push(3u8);
    this.build()
};

fn main() {}