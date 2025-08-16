use konst::array::ArrayBuilder;


fn main() {
    let _ = ArrayBuilder::<u8, 3, _>::of_copy::<5>();
    let _: ArrayBuilder<u8, 3, _> = ArrayBuilder::of_copy::<5>();

    let _ = ArrayBuilder::<u8, 3, _>::of_drop::<5>();
    let _: ArrayBuilder<u8, 3, _> = ArrayBuilder::of_drop::<5>();
}






