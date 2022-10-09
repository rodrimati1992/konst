const R: core::ops::Range<usize> = 0..10;
const S: &[u8] = &[3, 5];

const _: () = {
    konst::iter::eval!(R, any(|_| false,));
    konst::iter::eval!(R, any(|_,| false));
    konst::iter::eval!(R, any(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, all(|_| false,));
    konst::iter::eval!(R, all(|_,| false));
    konst::iter::eval!(R, all(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, filter(|_| false,));
    konst::iter::eval!(R, filter(|_,| false));
    konst::iter::eval!(R, filter(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, find(|_| false,));
    konst::iter::eval!(R, find(|_,| false));
    konst::iter::eval!(R, find(|_,| false,));
};


const _: () = {
    konst::iter::eval!(R, filter_map(|_| None,));
    konst::iter::eval!(R, filter_map(|_,| None));
    konst::iter::eval!(R, filter_map(|_,| None,));
};

const _: () = {
    konst::iter::eval!(R, find_map(|_| None,));
    konst::iter::eval!(R, find_map(|_,| None));
    konst::iter::eval!(R, find_map(|_,| None,));
};

const _: () = {
    konst::iter::eval!(R, flat_map(|_| R,));
    konst::iter::eval!(R, flat_map(|_,| R));
    konst::iter::eval!(R, flat_map(|_,| R,));
};

const _: () = {
    konst::iter::eval!(R, fold(0, |_, _| 0));
    konst::iter::eval!(R, fold(0, |_, _| 0,));
    konst::iter::eval!(R, fold(0, |_, _,| 0));
    konst::iter::eval!(R, fold(0, |_, _,| 0,));
};

const _: () = {
    konst::iter::eval!(R, for_each(|_| {}));
    konst::iter::eval!(R, for_each(|_| {},));
    konst::iter::eval!(R, for_each(|_,| {}));
    konst::iter::eval!(R, for_each(|_,| {},));
};

const _: () = {
    konst::iter::eval!(R, map(|_| true,));
    konst::iter::eval!(R, map(|_,| true));
    konst::iter::eval!(R, map(|_,| true,));
};

const _: () = {
    konst::iter::eval!(R, nth(0));
    konst::iter::eval!(R, nth(0,));
};

const _: () = {
    konst::iter::eval!(R, position(|_| false,));
    konst::iter::eval!(R, position(|_,| false));
    konst::iter::eval!(R, position(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, rfind(|_| false,));
    konst::iter::eval!(R, rfind(|_,| false));
    konst::iter::eval!(R, rfind(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, rposition(|_| false,));
    konst::iter::eval!(R, rposition(|_,| false));
    konst::iter::eval!(R, rposition(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, rfold(0, |_, _| 0));
    konst::iter::eval!(R, rfold(0, |_, _| 0,));
    konst::iter::eval!(R, rfold(0, |_, _,| 0));
    konst::iter::eval!(R, rfold(0, |_, _,| 0,));
};

const _: () = {
    konst::iter::eval!(R, skip(0));
    konst::iter::eval!(R, skip(0,));
};

const _: () = {
    konst::iter::eval!(R, take(0));
    konst::iter::eval!(R, take(0,));
};

const _: () = {
    konst::iter::eval!(R, skip_while(|_| false,));
    konst::iter::eval!(R, skip_while(|_,| false));
    konst::iter::eval!(R, skip_while(|_,| false,));
};

const _: () = {
    konst::iter::eval!(R, take_while(|_| false,));
    konst::iter::eval!(R, take_while(|_,| false));
    konst::iter::eval!(R, take_while(|_,| false,));
};


const _: () = {
    konst::iter::eval!(R, zip(S));
    konst::iter::eval!(R, zip(S,));
};





fn main(){}