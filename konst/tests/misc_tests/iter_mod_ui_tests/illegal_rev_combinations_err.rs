const _: Option<u8> = konst::iter::eval!{0..10,take(0),rev(),next()};

const _: Option<u8> = konst::iter::eval!{0..10,skip(0),rev(),next()};

const _: Option<u8> = konst::iter::eval!{0..10,take_while(|_| true),rev(),next()};

const _: Option<u8> = konst::iter::eval!{0..10,skip_while(|_| false),rev(),next()};

const _: Option<u8> = konst::iter::eval!{0..10,rev(),rev(),next()};


fn main() {}