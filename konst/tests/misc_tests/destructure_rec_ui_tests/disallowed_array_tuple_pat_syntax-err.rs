// tuple patterns

const _: () = { konst::destructure_rec!{ (a, .., b) = (1, 2, 3, 4) }; };

const _: () = { konst::destructure_rec!{ (a () b) = (1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ (a [] b) = (1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ (a ,, b) = (1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ (a @ () @ _) = (1, 2, 3) }; };

const _: () = { konst::destructure_rec!{ (a @ [] @ _) = (1, 2, 3) }; };

// array patterns

const _: () = { konst::destructure_rec!{ [foo[] aa] = [1, 2, 3, 4] }; };

const _: () = { konst::destructure_rec!{ [foo ,, []] = [1, 2, 3, 4] }; };

const _: () = { konst::destructure_rec!{ [[] []] = [1, 2, 3, 4] }; };

const FOO: u32 = 10;
const BAR: u32 = 10;

const _: () = { konst::destructure_rec!{ [FOO .. BAR] = [1, 2, 3, 4] }; };

const _: () = { konst::destructure_rec!{ [FOO ..] = [1, 2, 3, 4] }; };


fn main(){}