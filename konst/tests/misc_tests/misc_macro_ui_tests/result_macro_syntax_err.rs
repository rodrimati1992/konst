use konst::result;

const _: () = result::unwrap_or!();
const _: () = result::unwrap_or!(Ok(()),);

const _: () = result::unwrap_or_else!();
const _: () = result::unwrap_or_else!(Err(()),);
const _: () = result::unwrap_or_else!(Err(()), || ());

const _: () = result::unwrap_err_or_else!();
const _: () = result::unwrap_err_or_else!(Ok(()),);
const _: () = result::unwrap_err_or_else!(Ok(()), || ());

const _: Option<()> = result::ok!();

const _: Option<()> = result::err!();

const _: Result<bool, ()> = result::map!();
const _: Result<bool, ()> = result::map!(Ok(()),);
const _: Result<bool, ()> = result::map!(Ok(()), || false);

const _: Result<(), bool> = result::map_err!();
const _: Result<(), bool> = result::map_err!(Ok(()),);
const _: Result<(), bool> = result::map_err!(Ok(()), || false);

const _: Result<(), bool> = result::and_then!();
const _: Result<(), bool> = result::and_then!(Ok(()),);
const _: Result<(), bool> = result::and_then!(Ok(()), || Err(false));

const _: Result<(), bool> = result::or_else!();
const _: Result<(), bool> = result::or_else!(Ok(()),);
const _: Result<(), bool> = result::or_else!(Ok(()), || Ok(()));

const _: () = result::unwrap!();


fn main() {}