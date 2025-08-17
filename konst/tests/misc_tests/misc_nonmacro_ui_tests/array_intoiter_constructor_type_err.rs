use konst::array::IntoIter;


fn main() {
    let _ = IntoIter::<u8, 3, _>::of_copy::<5>([0u8; _]);
    let _: IntoIter<u8, 3, _> = IntoIter::of_copy::<5>([0u8; _]);

    let _ = IntoIter::<u8, 3, _>::of_drop::<5>([0u8; _]);
    let _: IntoIter<u8, 3, _> = IntoIter::of_drop::<5>([0u8; _]);

    let _: IntoIter<u8, 3, konst::drop_flavor::NonDrop> = IntoIter::of_copy([0u8; 3]).into_drop();

    let _: IntoIter<u8, 3, konst::drop_flavor::MayDrop> = IntoIter::of_drop([0u8; 3]).into_copy();

}






