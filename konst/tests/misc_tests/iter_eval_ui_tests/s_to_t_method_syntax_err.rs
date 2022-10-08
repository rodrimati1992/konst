const R: core::ops::Range<usize> = 0..10;

const _: () = {
    konst::iter::eval!(R, skip_while());
    konst::iter::eval!(R, skip_while(||false));
    konst::iter::eval!(R, skip_while(|_|));
    konst::iter::eval!(R, skip_while(|_| false, 100));
    konst::iter::eval!(R, skip_while(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, skip());
    konst::iter::eval!(R, skip(10, 20));
};

const _: () = {
    konst::iter::eval!(R, take());
    konst::iter::eval!(R, take(10, 20));
};

const _: () = {
    konst::iter::eval!(R, take_while());
    konst::iter::eval!(R, take_while(||false));
    konst::iter::eval!(R, take_while(|_|));
    konst::iter::eval!(R, take_while(|_| false, 100));
    konst::iter::eval!(R, take_while(|a, b|false));
};

const _: () = {
    konst::iter::eval!(R, zip());
    konst::iter::eval!(R, zip(R, 20));
};


fn main(){}
