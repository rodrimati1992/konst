const _: () = konst::iter::for_each!{};

const _: () = konst::iter::for_each!{_};

const _: () = konst::iter::for_each!{_ in};

const _: () = {
    let _ = konst::iter::for_each!{_ in 0..10,foobar() => };
};

const _: () = {
    let _ = konst::iter::for_each!{_ in 0..10,all(|_|false) => };
    let _ = konst::iter::for_each!{_ in 0..10,find(|_|false) => };
    let _ = konst::iter::for_each!{_ in 0..10,count() => };
};


fn main(){}