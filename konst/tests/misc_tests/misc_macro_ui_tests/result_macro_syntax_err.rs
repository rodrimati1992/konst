use konst::result;

const fn whatever1_return<T>(a0: impl Sized) -> T {
    std::mem::forget(a0);
    unimplemented!()
}


const _: () = result::unwrap_or!();
const _: () = result::unwrap_or!(Ok(()),);

const _: () = result::unwrap_or_else!();
const _: () = result::unwrap_or_else!(Err(()),);
const _: () = result::unwrap_or_else!(Err(()), || ());
const _: () = result::unwrap_or_else!(Err(()), {whatever1_return});

const _: () = result::unwrap_err_or_else!();
const _: () = result::unwrap_err_or_else!(Ok(()),);
const _: () = result::unwrap_err_or_else!(Ok(()), || ());
const _: () = result::unwrap_err_or_else!(Ok(()), {whatever1_return});

const _: Option<()> = result::ok!();

const _: Option<()> = result::err!();

const _: Result<bool, ()> = result::map!();
const _: Result<bool, ()> = result::map!(Ok(()),);
const _: Result<bool, ()> = result::map!(Ok(()), || false);
const _: Result<bool, ()> = result::map!(Ok(()), {whatever1_return});

const _: Result<(), bool> = result::map_err!();
const _: Result<(), bool> = result::map_err!(Ok(()),);
const _: Result<(), bool> = result::map_err!(Ok(()), || false);
const _: Result<(), bool> = result::map_err!(Ok(()), {whatever1_return});

const _: Result<(), bool> = result::and_then!();
const _: Result<(), bool> = result::and_then!(Ok(()),);
const _: Result<(), bool> = result::and_then!(Ok(()), || Err(false));
const _: Result<(), bool> = result::and_then!(Ok(()), {whatever1_return});

const _: Result<(), bool> = result::or_else!();
const _: Result<(), bool> = result::or_else!(Ok(()),);
const _: Result<(), bool> = result::or_else!(Ok(()), || Ok(()));
const _: Result<(), bool> = result::or_else!(Ok(()), {whatever1_return});

const _: () = result::unwrap!();


fn main() {}