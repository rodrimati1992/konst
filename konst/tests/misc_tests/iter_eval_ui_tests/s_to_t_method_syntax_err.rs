const R: core::ops::Range<usize> = 0..10;

const _: () = {
    konst::iter::eval!(R, skip_while(), next());
    konst::iter::eval!(R, skip_while(||false), next());
    konst::iter::eval!(R, skip_while(|_|), next());
    konst::iter::eval!(R, skip_while(|_| false, 100), next());
    konst::iter::eval!(R, skip_while(|a, b|false), next());
};

const _: () = {
    konst::iter::eval!(R, skip(), next());
    konst::iter::eval!(R, skip(10, 20), next());
};

const _: () = {
    konst::iter::eval!(R, take(), next());
    konst::iter::eval!(R, take(10, 20), next());
};

const _: () = {
    konst::iter::eval!(R, take_while(), next());
    konst::iter::eval!(R, take_while(||false), next());
    konst::iter::eval!(R, take_while(|_|), next());
    konst::iter::eval!(R, take_while(|_| false, 100), next());
    konst::iter::eval!(R, take_while(|a, b|false), next());
};

const _: () = {
    konst::iter::eval!(R, zip(), next());
    konst::iter::eval!(R, zip(R, 20), next());
};


fn main(){}
