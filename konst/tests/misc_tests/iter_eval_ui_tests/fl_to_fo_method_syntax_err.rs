const R: core::ops::Range<usize> = 0..10;
const S: &[[u8; 2]] = &[[3, 5], [8, 13]];

const _: () = {
    konst::iter::eval!(R, flat_map(), next());
    konst::iter::eval!(R, flat_map(||R), next());
    konst::iter::eval!(R, flat_map(|_|), next());
    konst::iter::eval!(R, flat_map(|_|R, 10), next());
    konst::iter::eval!(R, flat_map(|a, b|R), next());
};

const _: () = {
    konst::iter::eval!(S, flatten(), next());
    konst::iter::eval!(S, flatten(10), next());
};

const _: () = {
    konst::iter::eval!(R, fold());
    konst::iter::eval!(R, fold(0));
    konst::iter::eval!(R, fold(0,));
    konst::iter::eval!(R, fold(0, || 10));
    konst::iter::eval!(R, fold(0, |_| 10));
    konst::iter::eval!(R, fold(0, |_, _| ));
    konst::iter::eval!(R, fold(0, |_, _| 10, 100)); 
};

const _: () = {
    konst::iter::eval!(R, for_each());
    konst::iter::eval!(R, for_each(||None));
    konst::iter::eval!(R, for_each(|_|));
    konst::iter::eval!(R, for_each(|_|(), 100));
    konst::iter::eval!(R, for_each(|a, b|None));
};

fn main(){}
