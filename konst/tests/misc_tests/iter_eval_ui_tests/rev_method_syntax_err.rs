const R: core::ops::Range<usize> = 0..10;

const _: () = {
    konst::iter::eval!(R, rev(10), next());
};

const _: () = {
    konst::iter::eval!(R, rev(),rev(),next());
    konst::iter::eval!(R, rev(),rfind(|_| false));
    konst::iter::eval!(R, rev(),rposition(|_| false));
    konst::iter::eval!(R, rev(),rfold(0, |_, _| 10));
};

fn main(){}

