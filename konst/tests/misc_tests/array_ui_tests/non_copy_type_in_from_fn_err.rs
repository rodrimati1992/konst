struct Foo<T>(T);

const _: [Foo<usize>; 3] = konst::array::from_fn!(Foo);

const _: [String; 1] = konst::array::from_fn!(|_| String::new());

fn main(){}

