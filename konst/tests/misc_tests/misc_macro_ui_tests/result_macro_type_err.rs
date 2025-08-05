use konst::result;

const _: u8 = result::unwrap_or!((), ());

const _: u8 = result::unwrap_or_else!((), |_| false);

const _: u8 = result::unwrap_err_or_else!((), |_| false);

const _: u8 = result::ok!(());

const _: u8 = result::err!(());

const _: u8 = result::map!((), |_| false);

const _: u8 = result::map_err!((), |_| false);

const _: u8 = result::and_then!((), |_| false);

const _: u8 = result::or_else!((), |_| false);

const _: u8 = result::unwrap!(());


fn main() {}