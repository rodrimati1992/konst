const _: usize = {
    let parser = konst::Parser::new("foo bar baz").skip(3);
    konst::unwrap_ctx!(Err(parser.into_other_error(&"hello world!")))
};


fn main(){}