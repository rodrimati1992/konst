use konst::array::ArrayBuilder;


fn main() {
    let _ = ArrayBuilder::<u8, 3, _>::of_copy::<5>();
    let _: ArrayBuilder<u8, 3, _> = ArrayBuilder::of_copy::<5>();

    let _ = ArrayBuilder::<u8, 3, _>::of_drop::<5>();
    let _: ArrayBuilder<u8, 3, _> = ArrayBuilder::of_drop::<5>();

    let _: ArrayBuilder<u8, 3, konst::drop_flavor::NonDrop> = ArrayBuilder::of_copy().into_drop();

    let _: ArrayBuilder<u8, 3, konst::drop_flavor::MayDrop> = ArrayBuilder::of_drop().into_copy();
}






