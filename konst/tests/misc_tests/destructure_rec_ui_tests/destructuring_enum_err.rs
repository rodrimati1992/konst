use std::convert::Infallible as Never;



const _: () = { konst::destructure_rec!{Ok(x) = Ok::<_, Never>(10)}; };

const _: () = { konst::destructure_rec!{Some(x) = Some(10)}; };






fn main(){}