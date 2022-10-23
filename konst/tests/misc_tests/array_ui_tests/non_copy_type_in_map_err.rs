struct Foo<T>(T);

const _: [Foo<u32>; 3] = konst::array::map!([3, 5, 8], Foo);

const _: [Foo<String>; 1] = konst::array::map!([String::new()], Foo);

fn main(){}

