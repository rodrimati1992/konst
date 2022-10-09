const _: () = {
    konst::iter::eval!();
};

const _: () = {
    konst::iter::eval!(0usize..10, asdasdasd());
};

const _: () = {
    konst::iter::eval!(0usize..10, enumerate);
};

const _: () = {
    konst::iter::eval!(0usize..10, asdasdasd);
};

const _: () = {
    konst::iter::eval!(0usize..10, enumerate().foo());
};

const _: () = {
    konst::iter::eval!(0usize..10, enumerate(),nth(3),bar());
};


fn main(){}