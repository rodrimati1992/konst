const R: core::ops::Range<usize> = 0..10;
const S: &[u8] = &[3, 5];

const _: () = {
    konst::iter::eval!(R, any(|_| 10u8,));
};

const _: () = {
    konst::iter::eval!(R, all(|_| 10u8,));
};

const _: () = {
    konst::iter::eval!(R, filter(|_| 10u8,), next());
};

const _: () = {
    konst::iter::eval!(R, find(|_| 10u8,));
};


const _: () = {
    konst::iter::eval!(R, filter_map(|_| 10u8,), next());
};

const _: () = {
    konst::iter::eval!(R, find_map(|_| 10u8,));
};

const _: () = {
    konst::iter::eval!(
        R,flat_map(|_| R,),
            for_each(|x|{let _: u32 = x;}),
    );
};

const fn add(l: u32, r: u32) -> u32 {
    l + r
}


const _: () = {
    let _: () = konst::iter::eval!(R, reduce(add));
    let _: () = konst::iter::eval!(R, reduce(|x, y| add(x, y)));
};

const _: () = {
    let _: () = konst::iter::eval!(R, fold(0u32, add));
    let _: () = konst::iter::eval!(R, fold(0u32, |x, y| add(x, y)));
};

const _: () = {
    let _: () = konst::iter::eval!(R, rfold(0u32, add));
    let _: () = konst::iter::eval!(R, rfold(0u32, |accum, elem| add(accum, elem)));
};


const _: Option<u32> = {
    konst::iter::eval!(R, map(|_| true,),next())
};

const _: () = {
    konst::iter::eval!(R, nth(false));
};

const _: () = {
    konst::iter::eval!(R, position(|_| 10u8));
};

const _: () = {
    konst::iter::eval!(R, rfind(|_| 10u8));
};

const _: () = {
    konst::iter::eval!(R, step_by(10u8), next());
};

const _: () = {
    konst::iter::eval!(R, skip(10u8), next());
};

const _: () = {
    konst::iter::eval!(R, take(10u8), next());
};

const _: () = {
    konst::iter::eval!(R, skip_while(|_| None::<u32>,), next());
};

const _: () = {
    konst::iter::eval!(R, take_while(|_| None::<u32>,), next());
};


const _: Option<(usize, usize)> = konst::iter::eval!(R, zip(S), next());





fn main(){}