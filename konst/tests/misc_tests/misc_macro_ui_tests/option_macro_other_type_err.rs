use konst::option;

const _: () = option::unwrap_or!((), ());
const _: () = option::unwrap_or!(None::<u8>, ());
const _: () = option::unwrap_or!(None::<u8>, 0u8);

const _: () = option::unwrap_or_else!((), || ());
const _: () = option::unwrap_or_else!(None::<u8>, || ());
const _: () = option::unwrap_or_else!(None::<u8>, || 0u8);

const _: Option<()> = option::filter!((), |_| ());
const _: Option<()> = option::filter!(None::<u8>, |_| ());

const _: Option<(u8, u16)> = option::zip!((), ());

fn main() {}
