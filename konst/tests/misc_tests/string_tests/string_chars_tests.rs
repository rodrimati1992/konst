use konst::string;

// A string with all char lengths
const CHAR_LENS: &str = "fooñ个人bar\u{100000}b\u{10FFFF}az";

#[test]
fn chars_const_fn_test() {
    use konst::string::{Chars, RChars};
    const fn _is_const_callable<'a>(iter: Chars<'a>) {
        let _: Chars<'a> = iter.copy();
        let _: Option<(char, Chars<'a>)> = iter.copy().next();
        let _: Option<(char, Chars<'a>)> = iter.copy().next_back();
        let _: &'a str = iter.as_str();

        let rev: RChars<'a> = iter.rev();
        let _: Option<(char, RChars<'a>)> = rev.copy().next();
        let _: Option<(char, RChars<'a>)> = rev.next_back();
    }
}

#[test]
fn chars_test() {
    {
        let collected = collect_const_iter!(string::chars(CHAR_LENS));
        assert_eq!(collected, CHAR_LENS.chars().collect::<Vec<_>>());
    }
    {
        let mut kiter = string::chars(CHAR_LENS);
        let mut iter = CHAR_LENS.chars();
        let mut kelem;

        assert_eq!(kiter.as_str(), iter.as_str());

        while let Some(elem) = iter.next() {
            (kelem, kiter) = kiter.next().unwrap();
            assert_eq!(elem, kelem);
            assert_eq!(kiter.as_str(), iter.as_str());
        }

        assert!(kiter.next().is_none());
    }
    {
        let iter: string::Chars<'_> = string::chars(CHAR_LENS);
        let (f0, iter) = iter.next_back().unwrap();
        let (f1, iter) = iter.next_back().unwrap();
        let (f2, iter) = iter.next_back().unwrap();
        let (f3, iter) = iter.next_back().unwrap();
        let (f4, iter) = iter.next_back().unwrap();
        let (b0, iter) = iter.next().unwrap();
        let (b1, iter) = iter.next().unwrap();
        let (b2, iter) = iter.next().unwrap();
        let (b3, iter) = iter.next().unwrap();
        let (b4, iter) = iter.next().unwrap();

        assert_eq!(
            [f0, f1, f2, f3, f4],
            ['z', 'a', '\u{10FFFF}', 'b', '\u{100000}']
        );
        assert_eq!([b0, b1, b2, b3, b4], ['f', 'o', 'o', 'ñ', '个']);
        assert_eq!(iter.as_str(), "人bar");
    }
}

#[test]
fn chars_rev_test() {
    {
        let rev = collect_const_iter!(string::chars(CHAR_LENS).rev());
        assert_eq!(rev, CHAR_LENS.chars().rev().collect::<Vec<_>>());
    }
    {
        let iter: string::RChars<'_> = string::chars(CHAR_LENS).rev();
        let (f0, iter) = iter.next().unwrap();
        let (f1, iter) = iter.next().unwrap();
        let (f2, iter) = iter.next().unwrap();
        let (f3, iter) = iter.next().unwrap();
        let (f4, iter) = iter.next().unwrap();
        let (b0, iter) = iter.next_back().unwrap();
        let (b1, iter) = iter.next_back().unwrap();
        let (b2, iter) = iter.next_back().unwrap();
        let (b3, iter) = iter.next_back().unwrap();
        let (b4, iter) = iter.next_back().unwrap();

        assert_eq!(
            [f0, f1, f2, f3, f4],
            ['z', 'a', '\u{10FFFF}', 'b', '\u{100000}']
        );
        assert_eq!([b0, b1, b2, b3, b4], ['f', 'o', 'o', 'ñ', '个']);
        assert_eq!(iter.rev().as_str(), "人bar");
    }
}
