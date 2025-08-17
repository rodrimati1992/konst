use konst::option;

const _: Result<(), u16> = option::ok_or!((), 0u16);
const _: () = option::ok_or!(None::<u8>, 0u16);

const _: Result<(), u16> = option::ok_or_else!((), || 0u16);
const _: () = option::ok_or_else!(None::<u8>, || 0u16);

const _: Option<u8> = option::map!((), |_| 0u8);
const _: Option<u8> = option::map!(None::<()>, |__ @ 0u8..| 0u8);
const _: () = option::map!(None::<u8>, |_| ());

const _: Option<u16> = option::and_then!((), |_| None::<u16>);
const _: Option<u16> = option::and_then!(None::<()>, |__ @ 0u8..| None::<u16>);
const _: () = option::and_then!(None::<u8>, |__ @ 0u8..| None::<u16>);

const _: Option<u16> = option::or_else!((), || None::<u16>);
const _: Option<u16> = option::or_else!(None::<()>, || None::<u16>);
const _: () = option::or_else!(None::<u8>, || None::<u16>);


fn main() {}
