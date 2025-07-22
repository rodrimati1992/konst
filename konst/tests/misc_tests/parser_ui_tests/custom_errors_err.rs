const _: usize = {
    let mut parser = konst::Parser::new("foo bar baz");
    parser.skip(3);
    konst::result::unwrap!(Err(parser.to_other_error(&"hello world!")))
};


fn main(){}