use konst::slice;

struct NonCopy;

const _: () = {
    let arr = [const { NonCopy }; 4];

    slice::fill(&mut arr, NonCopy);
};



fn main() {}

