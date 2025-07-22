const _: usize = {
    let parser = konst::Parser::new("foo bar baz").skip(3);
    konst::result::unwrap!(Err(parser.to_other_error(&"hello world!")))
};


fn main(){}