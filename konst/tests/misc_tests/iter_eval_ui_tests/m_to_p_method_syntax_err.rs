const R: core::ops::Range<usize> = 0..10;

const _: () = {
    konst::iter::eval!(R, map());
    konst::iter::eval!(R, map(||false));
    konst::iter::eval!(R, map(|_|));
    konst::iter::eval!(R, map(|_| false, 100));
    konst::iter::eval!(R, map(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, next(10));
};

const _: () = {
    konst::iter::eval!(R, nth());
    konst::iter::eval!(R, nth(10, 20));
};

const _: () = {
    konst::iter::eval!(R, position());
    konst::iter::eval!(R, position(||false));
    konst::iter::eval!(R, position(|_|));
    konst::iter::eval!(R, position(|_| false, 100));
    konst::iter::eval!(R, position(|a, b|false));
};