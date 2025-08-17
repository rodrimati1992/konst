use konst::array::ArrayBuilder;


fn main() {
    let _ = ArrayBuilder::<String, 3, _>::of_drop().into_copy();
}

