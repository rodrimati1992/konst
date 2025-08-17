use konst::option;

const _: () = {
    let _: &mut u8 = option::get_or_insert!((), ());
    let _: &mut u8 = option::get_or_insert!(Some(()), ());
    let _: &mut u8 = option::get_or_insert!(&mut Some(()), 0u8);
};

const _: () = {
    let _: &mut u8 = option::get_or_insert_with!((), || 0u8);
    let _: &mut u8 = option::get_or_insert_with!(None::<()>, || 0u8);
    let _: &mut u8 = option::get_or_insert_with!(&mut None::<()>, || 0u8);
};

const _: () = {
    let _: &mut u8 = option::insert!((), ());
    let _: &mut u8 = option::insert!(Some(()), ());
    let _: &mut u8 = option::insert!(&mut Some(()), ());
    let _: &mut u8 = option::insert!(&mut Some(0u16), ());
};


const _: bool = option::is_some_and!((), |_| false);
const _: bool = option::is_some_and!(None::<()>, |__ @ 0u8..| false);
const _: bool = option::is_some_and!(None::<u8>, |_| ());

const _: bool = option::is_none_or!((), |_| false);
const _: bool = option::is_none_or!(None::<()>, |__ @ 0u8..| false);
const _: bool = option::is_none_or!(None::<u8>, |_| ());

fn main() {}