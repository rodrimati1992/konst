const R: core::ops::Range<usize> = 0..10;
const S: &[u8] = &[3, 5];

const _: () = {
    konst::iter::eval!(R, filter());
    konst::iter::eval!(R, filter(||false));
    konst::iter::eval!(R, filter(|_|));
    konst::iter::eval!(R, filter(|_| false, 100));
    konst::iter::eval!(R, filter(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, find());
    konst::iter::eval!(R, find(||false));
    konst::iter::eval!(R, find(|_| false, 100));
    konst::iter::eval!(R, find(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, filter_map());
    konst::iter::eval!(R, filter_map(||None));
    konst::iter::eval!(R, filter_map(|_|));
    konst::iter::eval!(R, filter_map(|_| None, 100));
    konst::iter::eval!(R, filter_map(|a, b|None));
};

const _: () = {
    konst::iter::eval!(R, find_map());
    konst::iter::eval!(R, find_map(||None));
    konst::iter::eval!(R, find_map(|_| None, 100));
    konst::iter::eval!(R, find_map(|a, b|None));
};


fn main(){}