use konst::iter::{ConstIntoIter, IsIteratorKind};

struct WrongItem;

impl ConstIntoIter for WrongItem {
    type Kind = IsIteratorKind;
    type IntoIter = Self;
    type Item = u8;
    const ITEMS_NEED_DROP: bool = false;
}

impl WrongItem {
    const fn next(&mut self) -> Option<u64> {
        None
    }
}

const _: () = {
    konst::iter::for_each!{_ in WrongItem => }
};


struct WrongIntoIter;

impl ConstIntoIter for WrongIntoIter {
    type Kind = IsIteratorKind;
    type IntoIter = konst::slice::IterCopied<'static, u8>;
    type Item = u8;
    const ITEMS_NEED_DROP: bool = false;
}

impl WrongIntoIter {
    const fn next(&mut self) -> Option<u8> {
        None
    }
}

const _: () = {
    konst::iter::for_each!{_ in WrongIntoIter => }
};



fn main() {}