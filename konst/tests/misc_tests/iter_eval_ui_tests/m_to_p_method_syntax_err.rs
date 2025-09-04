const R: core::ops::Range<usize> = 0..10;

const _: () = {
    // non-consuming eval is an error!
    konst::iter::eval!(R, map(|_| 0u8));

    konst::iter::eval!(R, map(), next());
    konst::iter::eval!(R, map(||false), next());
    konst::iter::eval!(R, map(|_|), next());
    konst::iter::eval!(R, map(|_| false, 100), next());
    konst::iter::eval!(R, map(|a, b|false), next());
};

const _: () = {
    // non-consuming eval is an error!
    konst::iter::eval!(R, map_while(|_| None::<u8>));
    
    konst::iter::eval!(R, map_while(),next());
    konst::iter::eval!(R, map_while(||None::<u8>),next());
    konst::iter::eval!(R, map_while(|_| None::<u8>, 100),next());
    konst::iter::eval!(R, map_while(|a, b|None::<u8>),next());
    konst::iter::eval!(R, map_while({whatever1_return}),next());
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