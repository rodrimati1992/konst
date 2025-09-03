
// takes drop type
const _: [(); 3] = konst::array::map_nd!([const { String::new() }; 3], std::mem::forget);

// returns drop type
const _: [String; 3] = konst::array::map_nd!([(); 3], |_| String::new());

// takes and returns drop type
const _: [Vec<u8>; 3] = konst::array::map_nd!([const { String::new() }; 3], String::into_bytes);


fn main() {}