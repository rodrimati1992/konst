use konst::string;

// A string with all char lengths
//
// (0, 'f')
// (1, 'o')
// (2, 'o')
// (3, 'ñ')
// (5, '个')
// (8, '人')
// (11, 'b')
// (12, 'a')
// (13, 'r')
// (14, '\u{100000}')
// (18, 'b')
// (19, '\u{10ffff}')
// (23, 'a')
// (24, 'z')
const CHAR_LENS: &str = "fooñ个人bar\u{100000}b\u{10FFFF}az";

#[test]
fn chars_const_fn_test() {
    use konst::string::{Chars, RChars};
    const fn _is_const_callable<'a>(iter: Chars<'a>) {
        let _: Chars<'a> = iter.copy();
        let _: Option<char> = iter.copy().next();
        let _: Option<char> = iter.copy().next_back();
        let _: &'a str = iter.as_str();

        let mut rev: RChars<'a> = iter.rev();
        let _: Option<char> = rev.copy().next();
        let _: Option<char> = rev.next_back();
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

        assert_eq!(kiter.as_str(), iter.as_str());

        while let Some(elem) = iter.next() {
            let kelem = kiter.next().unwrap();
            assert_eq!(elem, kelem);
            assert_eq!(kiter.as_str(), iter.as_str());
        }

        assert!(kiter.next().is_none());
    }
    {
        let mut iter: string::Chars<'_> = string::chars(CHAR_LENS);
        let f0 = iter.next_back().unwrap();
        let f1 = iter.next_back().unwrap();
        let f2 = iter.next_back().unwrap();
        let f3 = iter.next_back().unwrap();
        let f4 = iter.next_back().unwrap();
        let b0 = iter.next().unwrap();
        let b1 = iter.next().unwrap();
        let b2 = iter.next().unwrap();
        let b3 = iter.next().unwrap();
        let b4 = iter.next().unwrap();

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
        let mut iter: string::RChars<'_> = string::chars(CHAR_LENS).rev();
        let f0 = iter.next().unwrap();
        let f1 = iter.next().unwrap();
        let f2 = iter.next().unwrap();
        let f3 = iter.next().unwrap();
        let f4 = iter.next().unwrap();
        let b0 = iter.next_back().unwrap();
        let b1 = iter.next_back().unwrap();
        let b2 = iter.next_back().unwrap();
        let b3 = iter.next_back().unwrap();
        let b4 = iter.next_back().unwrap();

        assert_eq!(
            [f0, f1, f2, f3, f4],
            ['z', 'a', '\u{10FFFF}', 'b', '\u{100000}']
        );
        assert_eq!([b0, b1, b2, b3, b4], ['f', 'o', 'o', 'ñ', '个']);
        assert_eq!(iter.rev().as_str(), "人bar");
    }
}

#[test]
fn char_indices_const_fn_test() {
    use konst::string::{CharIndices, RCharIndices};
    const fn _is_const_callable<'a>(iter: CharIndices<'a>) {
        let _: CharIndices<'a> = iter.copy();
        let _: Option<(usize, char)> = iter.copy().next();
        let _: Option<(usize, char)> = iter.copy().next_back();
        let _: &'a str = iter.as_str();

        let mut rev: RCharIndices<'a> = iter.rev();
        let _: Option<(usize, char)> = rev.copy().next();
        let _: Option<(usize, char)> = rev.next_back();
    }
}

#[test]
fn char_indices_test() {
    {
        let collected = collect_const_iter!(string::char_indices(CHAR_LENS));
        assert_eq!(collected, CHAR_LENS.char_indices().collect::<Vec<_>>());
    }
    {
        let mut kiter = string::char_indices(CHAR_LENS);
        let mut iter = CHAR_LENS.char_indices();

        assert_eq!(kiter.as_str(), iter.as_str());

        while let Some(elem) = iter.next() {
            let kelem = kiter.next().unwrap();
            assert_eq!(elem, kelem);
            assert_eq!(kiter.as_str(), iter.as_str());
        }

        assert!(kiter.next().is_none());
    }
    {
        let mut iter: string::CharIndices<'_> = string::char_indices(CHAR_LENS);
        let f0 = iter.next_back().unwrap();
        let f1 = iter.next_back().unwrap();
        let f2 = iter.next_back().unwrap();
        let f3 = iter.next_back().unwrap();
        let f4 = iter.next_back().unwrap();
        let b0 = iter.next().unwrap();
        let b1 = iter.next().unwrap();
        let b2 = iter.next().unwrap();
        let b3 = iter.next().unwrap();
        let b4 = iter.next().unwrap();

        assert_eq!(
            [f0, f1, f2, f3, f4],
            [
                (24, 'z'),
                (23, 'a'),
                (19, '\u{10ffff}'),
                (18, 'b'),
                (14, '\u{100000}')
            ]
        );

        assert_eq!(
            [b0, b1, b2, b3, b4],
            [(0, 'f'), (1, 'o'), (2, 'o'), (3, 'ñ'), (5, '个')]
        );
        assert_eq!(iter.as_str(), "人bar");
    }
}

#[test]
fn char_indices_rev_test() {
    {
        let rev = collect_const_iter!(string::char_indices(CHAR_LENS).rev());
        assert_eq!(rev, CHAR_LENS.char_indices().rev().collect::<Vec<_>>());
    }
    {
        let mut iter: string::RCharIndices<'_> = string::char_indices(CHAR_LENS).rev();
        let f0 = iter.next().unwrap();
        let f1 = iter.next().unwrap();
        let f2 = iter.next().unwrap();
        let f3 = iter.next().unwrap();
        let f4 = iter.next().unwrap();
        let b0 = iter.next_back().unwrap();
        let b1 = iter.next_back().unwrap();
        let b2 = iter.next_back().unwrap();
        let b3 = iter.next_back().unwrap();
        let b4 = iter.next_back().unwrap();

        assert_eq!(
            [f0, f1, f2, f3, f4],
            [
                (24, 'z'),
                (23, 'a'),
                (19, '\u{10ffff}'),
                (18, 'b'),
                (14, '\u{100000}')
            ]
        );

        assert_eq!(
            [b0, b1, b2, b3, b4],
            [(0, 'f'), (1, 'o'), (2, 'o'), (3, 'ñ'), (5, '个')]
        );
        assert_eq!(iter.rev().as_str(), "人bar");
    }
}
