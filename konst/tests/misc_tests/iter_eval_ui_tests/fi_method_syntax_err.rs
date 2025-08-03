const R: core::ops::Range<usize> = 0..10;
const S: &[u8] = &[3, 5];

const _: () = {
    konst::iter::eval!(R, filter(), next());
    konst::iter::eval!(R, filter(||false), next());
    konst::iter::eval!(R, filter(|_|), next());
    konst::iter::eval!(R, filter(|_| false, 100), next());
    konst::iter::eval!(R, filter(|a, b|false), next());
};

const _: () = {
    konst::iter::eval!(R, find());
    konst::iter::eval!(R, find(||false));
    konst::iter::eval!(R, find(|_| false, 100));
    konst::iter::eval!(R, find(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, filter_map(), next());
    konst::iter::eval!(R, filter_map(||None), next());
    konst::iter::eval!(R, filter_map(|_|), next());
    konst::iter::eval!(R, filter_map(|_| None, 100), next());
    konst::iter::eval!(R, filter_map(|a, b|None), next());
};

const _: () = {
    konst::iter::eval!(R, find_map());
    konst::iter::eval!(R, find_map(||None));
    konst::iter::eval!(R, find_map(|_| None, 100));
    konst::iter::eval!(R, find_map(|a, b|None));
};


fn main(){}