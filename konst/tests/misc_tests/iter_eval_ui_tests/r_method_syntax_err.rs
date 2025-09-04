const R: core::ops::Range<usize> = 0..10;
const S: &[[u8; 2]] = &[[3, 5], [8, 13]];

const fn whatever1_return<T>(a0: impl Sized) -> T {
    std::mem::forget(a0);
    unimplemented!()
}

const fn whatever2_return<T>(a0: impl Sized, a1: impl Sized) -> T {
    std::mem::forget(a0);
    std::mem::forget(a1);
    unimplemented!()
}

const _: () = {
    konst::iter::eval!(R, rfind());
    konst::iter::eval!(R, rfind(||false));
    konst::iter::eval!(R, rfind(|_| false, 100));
    konst::iter::eval!(R, rfind(|a, b|false));
    konst::iter::eval!(R, rfind({whatever1_return}));
};

const _: () = {
    konst::iter::eval!(R, rfold());
    konst::iter::eval!(R, rfold(0));
    konst::iter::eval!(R, rfold(0,));
    konst::iter::eval!(R, rfold(0, || 10));
    konst::iter::eval!(R, rfold(0, |_| 10));
    konst::iter::eval!(R, rfold(0, |_, _| ));
    konst::iter::eval!(R, rfold(0, {whatever2_return::<u8>})); 
};

const _: () = {
    konst::iter::eval!(R, reduce());
    konst::iter::eval!(R, reduce(|| 10));
    konst::iter::eval!(R, reduce(|_| 10));
    konst::iter::eval!(R, reduce(|_, _| ));
    konst::iter::eval!(R, reduce({whatever2_return::<u8>})); 
};

fn main(){}
