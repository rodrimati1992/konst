use konst::slice::{
    bytes_contain, bytes_end_with, bytes_find, bytes_find_keep, bytes_find_skip, bytes_rcontain,
    bytes_rfind, bytes_rfind_keep, bytes_rfind_skip, bytes_start_with, bytes_strip_prefix,
    bytes_strip_suffix, bytes_trim_end_matches, bytes_trim_matches,
    bytes_trim_start_matches, slice_from, slice_up_to,
};

// This doesn't use unsafe
#[cfg(not(miri))]
#[test]
fn find_test() {
    #[track_caller]
    fn ass(
        left: &[u8],
        right: &[u8],
        finds: &[(usize, Option<usize>)],
        rfinds: &[(usize, Option<usize>)],
    ) {
        for (offset, opt) in finds.iter().copied() {
            assert_eq!(
                bytes_find(slice_from(left, offset), right).map(|i| i + offset),
                opt,
                "in find, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_find(slice_from(b"", offset), right).map(|i| i + offset),
                    None,
                    "in find empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_find(slice_from(left, offset), b"").map(|i| i + offset),
                    Some(offset),
                    "in find empty right, offset: {}",
                    offset
                );
            }
        }
        for (offset, opt) in rfinds.iter().copied() {
            assert_eq!(
                bytes_rfind(slice_up_to(left, offset.saturating_add(1)), right),
                opt,
                "in rfind, offset: {}",
                offset
            );

            if !right.is_empty() {
                assert_eq!(
                    bytes_rfind(slice_up_to(b"", offset.saturating_add(1)), right),
                    None,
                    "in rfind empty left, offset: {}",
                    offset
                );
            }
            if offset < left.len() {
                assert_eq!(
                    bytes_rfind(slice_up_to(left, offset.saturating_add(1)), b""),
                    Some(offset),
                    "in rfind empty right, offset: {}",
                    offset
                );
            }
        }
    }

    ass(
        b"foo-bar-baz-foo---",
        b"foo",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (17, Some(12)),
            (15, Some(12)),
            (14, Some(12)),
            (13, Some(0)),
            (12, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (1, None),
            (0, None),
        ],
    );

    ass(
        b"foo-bar-baz-foo---",
        b"f",
        &[
            (0, Some(0)),
            (1, Some(12)),
            (4, Some(12)),
            (12, Some(12)),
            (13, None),
            (17, None),
            (18, None),
            (!0, None),
        ],
        &[
            (!0, Some(12)),
            (18, Some(12)),
            (17, Some(12)),
            (14, Some(12)),
            (13, Some(12)),
            (12, Some(12)),
            (11, Some(0)),
            (3, Some(0)),
            (3, Some(0)),
            (2, Some(0)),
            (0, Some(0)),
        ],
    );

    // Tests overlapping patterns
    ass(
        b"lawlawnawn--awn-lawlawn",
        b"lawn",
        &[
            (0, Some(3)),
            (1, Some(3)),
            (2, Some(3)),
            (3, Some(3)),
            (4, Some(19)),
            (16, Some(19)),
            (17, Some(19)),
            (18, Some(19)),
            (20, None),
            (22, None),
            (23, None),
            (!0, None),
        ],
        &[
            (!0, Some(19)),
            (23, Some(19)),
            (22, Some(19)),
            (20, Some(3)),
            (18, Some(3)),
            (17, Some(3)),
            (16, Some(3)),
            (8, Some(3)),
            (7, Some(3)),
            (6, Some(3)),
            (5, None),
            (4, None),
            (3, None),
            (2, None),
            (1, None),
            (0, None),
        ],
    );
}

#[test]
fn bytes_contain_test() {
    assert_eq!(bytes_contain(b"foo bar baz", "bar"), true);
    assert_eq!(bytes_contain(b"foo bar baz", "qux"), false);

    assert_eq!(bytes_contain(b"foo bar baz", b"baz" as &[u8; 3]), true);
    assert_eq!(bytes_contain(b"foo bar baz", b"qux" as &[u8; 3]), false);

    assert_eq!(bytes_contain(b"foo bar baz", b"baz" as &[u8]), true);
    assert_eq!(bytes_contain(b"foo bar baz", b"qux" as &[u8]), false);

    assert_eq!(bytes_contain(b"foo bar baz", &'o'), true);
    assert_eq!(bytes_contain(b"foo bar baz", &'q'), false);
}

#[test]
fn bytes_rcontain_test() {
    assert_eq!(bytes_rcontain(b"foo bar baz", "bar"), true);
    assert_eq!(bytes_rcontain(b"foo bar baz", "qux"), false);

    assert_eq!(bytes_rcontain(b"foo bar baz", b"baz" as &[u8; 3]), true);
    assert_eq!(bytes_rcontain(b"foo bar baz", b"qux" as &[u8; 3]), false);

    assert_eq!(bytes_rcontain(b"foo bar baz", b"baz" as &[u8]), true);
    assert_eq!(bytes_rcontain(b"foo bar baz", b"qux" as &[u8]), false);

    assert_eq!(bytes_rcontain(b"foo bar baz", &'o'), true);
    assert_eq!(bytes_rcontain(b"foo bar baz", &'q'), false);
}

#[test]
fn bytes_end_with_test() {
    assert_eq!(bytes_end_with(b"foo bar baz", "baz"), true);
    assert_eq!(bytes_end_with(b"foo bar baz", "qux"), false);

    assert_eq!(bytes_end_with(b"foo that", b"that" as &[u8; 4]), true);
    assert_eq!(bytes_end_with(b"foo that", b"asd" as &[u8; 3]), false);

    assert_eq!(bytes_end_with(b"foo that", b"that" as &[u8]), true);
    assert_eq!(bytes_end_with(b"foo that", b"asd" as &[u8]), false);

    assert_eq!(bytes_end_with(b"foo bar baz", &'z'), true);
    assert_eq!(bytes_end_with(b"foo bar baz", &'q'), false);
}

#[test]
fn find_basic_test() {
    assert_eq!(bytes_find(b"foo bar baz", "bar"), Some(4));
    assert_eq!(bytes_find(b"foo bar baz", "qux"), None);

    assert_eq!(bytes_find(b"foo bar baz", b"baz" as &[u8; 3]), Some(8));
    assert_eq!(bytes_find(b"foo bar baz", b"qux" as &[u8; 3]), None);

    assert_eq!(bytes_find(b"foo bar baz", b"baz" as &[u8]), Some(8));
    assert_eq!(bytes_find(b"foo bar baz", b"qux" as &[u8]), None);

    assert_eq!(bytes_find(b"foo bar baz", &'o'), Some(1));
    assert_eq!(bytes_find(b"foo bar baz", &'q'), None);
}
#[test]
fn bytes_rfind_test() {
    assert_eq!(bytes_rfind(b"foo bar baz", "ba"), Some(8));
    assert_eq!(bytes_rfind(b"foo bar baz", "qux"), None);

    assert_eq!(bytes_rfind(b"foo bar baz", b"ba" as &[u8; 2]), Some(8));
    assert_eq!(bytes_rfind(b"foo bar baz", b"qux" as &[u8; 3]), None);

    assert_eq!(bytes_rfind(b"foo bar baz", b"ba" as &[u8]), Some(8));
    assert_eq!(bytes_rfind(b"foo bar baz", b"qux" as &[u8]), None);

    assert_eq!(bytes_rfind(b"foo bar baz", &'o'), Some(2));
    assert_eq!(bytes_rfind(b"foo bar baz", &'q'), None);
}

#[test]
fn bytes_find_keep_test() {
    assert_eq!(
        bytes_find_keep(b"foo bar baz", "bar"),
        Some(&b"bar baz"[..])
    );
    assert_eq!(bytes_find_keep(b"foo bar baz", "qux"), None);

    assert_eq!(
        bytes_find_keep(b"foo baz that", b"baz" as &[u8; 3]),
        Some(&b"baz that"[..])
    );
    assert_eq!(bytes_find_keep(b"foo baz that", b"qux" as &[u8; 3]), None);

    assert_eq!(
        bytes_find_keep(b"foo baz that", b"baz" as &[u8]),
        Some(&b"baz that"[..])
    );
    assert_eq!(bytes_find_keep(b"foo baz that", b"qux" as &[u8]), None);

    assert_eq!(bytes_find_keep(b"foo bar baz", &'b'), Some(&b"bar baz"[..]));
    assert_eq!(bytes_find_keep(b"foo bar baz", &'q'), None);
}

#[test]
fn bytes_rfind_keep_test() {
    assert_eq!(
        bytes_rfind_keep(b"foo bar baz", "ba"),
        Some(&b"foo bar ba"[..])
    );
    assert_eq!(bytes_rfind_keep(b"foo bar baz", "qux"), None);

    assert_eq!(
        bytes_rfind_keep(b"foo baz that", b"a" as &[u8; 1]),
        Some(&b"foo baz tha"[..])
    );
    assert_eq!(bytes_rfind_keep(b"foo baz that", b"qux" as &[u8; 3]), None);

    assert_eq!(
        bytes_rfind_keep(b"foo baz that", b"a" as &[u8]),
        Some(&b"foo baz tha"[..])
    );
    assert_eq!(bytes_rfind_keep(b"foo baz that", b"qux" as &[u8]), None);

    assert_eq!(
        bytes_rfind_keep(b"foo bar baz", &'b'),
        Some(&b"foo bar b"[..])
    );
    assert_eq!(bytes_rfind_keep(b"foo bar baz", &'q'), None);
}

#[test]
fn bytes_rfind_skip_test() {
    assert_eq!(
        bytes_rfind_skip(b"foo bar baz", "ba"),
        Some(&b"foo bar "[..])
    );
    assert_eq!(bytes_rfind_skip(b"foo bar baz", "qux"), None);

    assert_eq!(
        bytes_rfind_skip(b"foo baz that", b"a" as &[u8; 1]),
        Some(&b"foo baz th"[..])
    );
    assert_eq!(bytes_rfind_skip(b"foo baz that", b"qux" as &[u8; 3]), None);

    assert_eq!(
        bytes_rfind_skip(b"foo baz that", b"a" as &[u8]),
        Some(&b"foo baz th"[..])
    );
    assert_eq!(bytes_rfind_skip(b"foo baz that", b"qux" as &[u8]), None);

    assert_eq!(
        bytes_rfind_skip(b"foo bar abcde", &'c'),
        Some(&b"foo bar ab"[..])
    );
    assert_eq!(bytes_rfind_skip(b"foo bar abcde", &'q'), None);
}

#[test]
fn bytes_find_skip_test() {
    assert_eq!(bytes_find_skip(b"foo bar baz", "bar"), Some(&b" baz"[..]));
    assert_eq!(bytes_find_skip(b"foo bar baz", "qux"), None);

    assert_eq!(
        bytes_find_skip(b"foo baz that", b"baz" as &[u8; 3]),
        Some(&b" that"[..])
    );
    assert_eq!(bytes_find_skip(b"foo baz that", b"qux" as &[u8; 3]), None);

    assert_eq!(
        bytes_find_skip(b"foo baz that", b"baz" as &[u8]),
        Some(&b" that"[..])
    );
    assert_eq!(bytes_find_skip(b"foo baz that", b"qux" as &[u8]), None);

    assert_eq!(bytes_find_skip(b"foo bar baz", &'b'), Some(&b"ar baz"[..]));
    assert_eq!(bytes_find_skip(b"foo bar baz", &'q'), None);
}

#[test]
fn bytes_start_with_test() {
    assert_eq!(bytes_start_with(b"what is that", "what"), true);
    assert_eq!(bytes_start_with(b"what is that", "qux"), false);

    assert_eq!(bytes_start_with(b"I'm, dunno?", b"I'm" as &[u8; 3]), true);
    assert_eq!(bytes_start_with(b"I'm, dunno?", b"asd" as &[u8; 3]), false);

    assert_eq!(bytes_start_with(b"I'm, dunno?", b"I'm" as &[u8]), true);
    assert_eq!(bytes_start_with(b"I'm, dunno?", b"asd" as &[u8]), false);

    assert_eq!(bytes_start_with(b"foo bar baz", &'f'), true);
    assert_eq!(bytes_start_with(b"goo bar baz", &'g'), true);
    assert_eq!(bytes_start_with(b"foo bar baz", &'b'), false);
}

#[test]
fn bytes_strip_prefix_test() {
    assert_eq!(
        bytes_strip_prefix(b"what is that", "what"),
        Some(&b" is that"[..])
    );
    assert_eq!(bytes_strip_prefix(b"what is that", "qux"), None);

    assert_eq!(
        bytes_strip_prefix(b"I'm, dunno?", b"I'm" as &[u8; 3]),
        Some(&b", dunno?"[..])
    );
    assert_eq!(bytes_strip_prefix(b"I'm, dunno?", b"asd" as &[u8; 3]), None);

    assert_eq!(
        bytes_strip_prefix(b"I'm, dunno?", b"I'm" as &[u8]),
        Some(&b", dunno?"[..])
    );
    assert_eq!(bytes_strip_prefix(b"I'm, dunno?", b"asd" as &[u8]), None);

    assert_eq!(
        bytes_strip_prefix(b"foo bar baz", &'f'),
        Some(&b"oo bar baz"[..])
    );
    assert_eq!(
        bytes_strip_prefix(b"goo bar baz", &'g'),
        Some(&b"oo bar baz"[..])
    );
    assert_eq!(bytes_strip_prefix(b"foo bar baz", &'b'), None);
}

#[test]
fn bytes_strip_suffix_test() {
    assert_eq!(
        bytes_strip_suffix(b"foo bar baz", "baz"),
        Some(&b"foo bar "[..])
    );
    assert_eq!(bytes_strip_suffix(b"foo bar baz", "qux"), None);

    assert_eq!(
        bytes_strip_suffix(b"foo that", b"that" as &[u8; 4]),
        Some(&b"foo "[..])
    );
    assert_eq!(bytes_strip_suffix(b"foo that", b"asd" as &[u8; 3]), None);

    assert_eq!(
        bytes_strip_suffix(b"foo that", b"that" as &[u8]),
        Some(&b"foo "[..])
    );
    assert_eq!(bytes_strip_suffix(b"foo that", b"asd" as &[u8]), None);

    assert_eq!(
        bytes_strip_suffix(b"foo bar baz", &'z'),
        Some(&b"foo bar ba"[..])
    );
    assert_eq!(bytes_strip_suffix(b"foo bar baz", &'q'), None);
}

#[test]
fn bytes_trim_matches_test() {
    assert_eq!(bytes_trim_matches(b"-----foo-----", "--"), b"-foo-");
    assert_eq!(bytes_trim_matches(b"hehe", "qux"), b"hehe");

    assert_eq!(
        bytes_trim_matches(b"-----foo-----", b"--" as &[u8; 2]),
        b"-foo-"
    );
    assert_eq!(bytes_trim_matches(b"hehe", b"qu" as &[u8; 2]), b"hehe");

    assert_eq!(
        bytes_trim_matches(b"-----foo-----", b"--" as &[u8]),
        b"-foo-"
    );
    assert_eq!(bytes_trim_matches(b"hehe", b"qu" as &[u8]), b"hehe");

    assert_eq!(bytes_trim_matches(b"fffffuuufffff", &'f'), b"uuu");
    assert_eq!(bytes_trim_matches(b"noooo", &'b'), b"noooo");
}

#[test]
fn bytes_trim_start_matches_test() {
    assert_eq!(
        bytes_trim_start_matches(b"-----foo-----", "--"),
        b"-foo-----"
    );
    assert_eq!(bytes_trim_start_matches(b"hehe", "qux"), b"hehe");

    assert_eq!(
        bytes_trim_start_matches(b"-----foo-----", b"--" as &[u8; 2]),
        b"-foo-----"
    );
    assert_eq!(
        bytes_trim_start_matches(b"hehe", b"qu" as &[u8; 2]),
        b"hehe"
    );

    assert_eq!(
        bytes_trim_start_matches(b"-----foo-----", b"--" as &[u8]),
        b"-foo-----"
    );
    assert_eq!(bytes_trim_start_matches(b"hehe", b"qu" as &[u8]), b"hehe");

    assert_eq!(
        bytes_trim_start_matches(b"fffffuuufffff", &'f'),
        b"uuufffff"
    );
    assert_eq!(bytes_trim_start_matches(b"noooo", &'b'), b"noooo");
}

#[test]
fn bytes_trim_end_matches_test() {
    assert_eq!(bytes_trim_end_matches(b"-----foo-----", "--"), b"-----foo-");
    assert_eq!(bytes_trim_end_matches(b"hehe", "qux"), b"hehe");

    assert_eq!(
        bytes_trim_end_matches(b"-----foo-----", b"--" as &[u8; 2]),
        b"-----foo-"
    );
    assert_eq!(bytes_trim_end_matches(b"hehe", b"qu" as &[u8; 2]), b"hehe");

    assert_eq!(
        bytes_trim_end_matches(b"-----foo-----", b"--" as &[u8]),
        b"-----foo-"
    );
    assert_eq!(bytes_trim_end_matches(b"hehe", b"qu" as &[u8]), b"hehe");

    assert_eq!(bytes_trim_end_matches(b"fffffuuufffff", &'f'), b"fffffuuu");
    assert_eq!(bytes_trim_end_matches(b"noooo", &'b'), b"noooo");
}
