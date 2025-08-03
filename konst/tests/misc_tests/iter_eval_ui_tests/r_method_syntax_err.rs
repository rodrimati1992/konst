const R: core::ops::Range<usize> = 0..10;
const S: &[[u8; 2]] = &[[3, 5], [8, 13]];

const _: () = {
    konst::iter::eval!(R, rfind());
    konst::iter::eval!(R, rfind(||false));
    konst::iter::eval!(R, rfind(|_| false, 100));
    konst::iter::eval!(R, rfind(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, rfold());
    konst::iter::eval!(R, rfold(0));
    konst::iter::eval!(R, rfold(0,));
    konst::iter::eval!(R, rfold(0, || 10));
    konst::iter::eval!(R, rfold(0, |_| 10));
    konst::iter::eval!(R, rfold(0, |_, _| ));
    konst::iter::eval!(R, rfold(0, |_, _| 10, 100)); 
};

fn main(){}
