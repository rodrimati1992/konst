use konst::option;


const fn whatever0_return<T>() -> T {
    unimplemented!()
}
const fn whatever1_return<T>(a0: impl Sized) -> T {
    std::mem::forget(a0);
    unimplemented!()
}


const _: u8 = option::unwrap_or!();
const _: u8 = option::unwrap_or!(None,);

const _: u8 = option::unwrap_or_else!();
const _: u8 = option::unwrap_or_else!(None,);
const _: u8 = option::unwrap_or_else!(None, |_| 0u8);
const _: u8 = option::unwrap_or_else!(None::<u8>, {whatever0_return});

const _: Result<(), u8> = option::ok_or!();
const _: Result<(), u8> = option::ok_or!(None,);

const _: Result<(), u8> = option::ok_or_else!();
const _: Result<(), u8> = option::ok_or_else!(None::<()>,);
const _: Result<(), u8> = option::ok_or_else!(None::<()>, |_| 0u8);
const _: Result<(), u8> = option::ok_or_else!(None::<()>, {whatever0_return::<u8>});

const _: Option<u8> = option::map!();
const _: Option<u8> = option::map!(None::<()>,);
const _: Option<u8> = option::map!(None::<()>, || 0u8);
const _: Option<u8> = option::map!(None::<()>, {whatever1_return});

const _: Option<u8> = option::and_then!();
const _: Option<u8> = option::and_then!(None::<()>,);
const _: Option<u8> = option::and_then!(None::<()>, || None);
const _: Option<u8> = option::and_then!(None::<()>, {whatever0_return});

const _: Option<u8> = option::or_else!();
const _: Option<u8> = option::or_else!(None::<u8>,);
const _: Option<u8> = option::or_else!(None::<u8>, |_| None);
const _: Option<u8> = option::or_else!(None::<u8>, {whatever0_return});

const _: Option<u8> = option::filter!();
const _: Option<u8> = option::filter!(None::<u8>,);
const _: Option<u8> = option::filter!(None::<u8>, || false);
const _: Option<u8> = option::filter!(None::<u8>, {whatever1_return});

const _: () = {
    let _: &mut u8 = option::get_or_insert!();
    let _: &mut u8 = option::get_or_insert!(&mut None,);
};

const _: () = {
    let _: &mut u8 = option::get_or_insert_with!();
    let _: &mut u8 = option::get_or_insert_with!(&mut None,);
    let _: &mut u8 = option::get_or_insert_with!(&mut None, |_| 0u8);
    let _: &mut u8 = option::get_or_insert_with!(&mut None, ||);
    let _: &mut u8 = option::get_or_insert_with!(&mut None, {whatever0_return});
};

const _: () = {
    let _: &mut u8 = option::insert!();
    let _: &mut u8 = option::insert!(&mut None,);
};

const _: Option<(u8, u16)> = option::zip!();
const _: Option<(u8, u16)> = option::zip!(None,);

const _: bool = option::is_some_and!();
const _: bool = option::is_some_and!(None,);
const _: bool = option::is_some_and!(None, || false);
const _: bool = option::is_some_and!(None, {whatever1_return});

const _: bool = option::is_none_or!();
const _: bool = option::is_none_or!(None,);
const _: bool = option::is_none_or!(None, || false);
const _: bool = option::is_none_or!(None, {whatever1_return});


fn main() {}