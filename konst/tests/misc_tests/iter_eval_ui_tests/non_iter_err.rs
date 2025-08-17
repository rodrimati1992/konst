const _: () = {
    konst::iter::eval!((), map(|_| 10),fold(0, |l, r| l + r));
};

fn main(){}