const R: core::ops::Range<usize> = 0..10;
const S: &[u8] = &[3, 5];

const fn whatever1_return<T>(a0: impl Sized) -> T {
    std::mem::forget(a0);
    unimplemented!()
}


const _: () = {
    konst::iter::eval!(R, any());
    konst::iter::eval!(R, any(||false));
    konst::iter::eval!(R, any(|_|));
    konst::iter::eval!(R, any(|_|(), 100));
    konst::iter::eval!(R, any(|a, b|false));
    konst::iter::eval!(R, any({whatever1_return}));
};

const _: () = {
    konst::iter::eval!(R, all());
    konst::iter::eval!(R, all(||false));
    konst::iter::eval!(R, all(|_|(), 100));
    konst::iter::eval!(R, all(|a, b|false));
    konst::iter::eval!(R, all({whatever1_return}));
};

const _: () = {
    konst::iter::eval!(S, copied(10), next());
};

const _: () = {
    konst::iter::eval!(R, count(10));
};

const _: () = {
    konst::iter::eval!(R, enumerate(10), next());
};


fn main(){}
