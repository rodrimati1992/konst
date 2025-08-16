use konst::array::IntoIter;


fn main() {
    let _ = IntoIter::<String, 1, _>::of_drop([String::new()]).into_copy();
}

