use konst::string;

use super::test_utils::must_panic;

// 0..1: 'f'
// 1..2: 'o'
// 2..3: 'o'
// 3..5: 'ñ'
// 5..8: '个'
// 8..11: '人'
// 11..12: 'b'
// 12..13: 'a'
// 13..14: 'r'
// 14..18: '\u{100000}'
// 18..19: 'b'
// 19..20: 'a'
// 20..21: 'z'
const CHAR_LENS: &str = "fooñ个人bar\u{100000}baz";
const LEN: usize = CHAR_LENS.len();

const INVALID_INDICES: &[usize] = &[4, 6, 7, 9, 10, 15, 16, 17];
const OOB_INDICES: &[usize] = &[LEN + 1, LEN + 10, !0 - 1, !0];

#[test]
fn test_char_boundary_inside() {
    for start in 0..=CHAR_LENS.len() {
        for end in 0..=CHAR_LENS.len() {
            let is_inv_start = INVALID_INDICES.contains(&start);
            let is_inv_end = INVALID_INDICES.contains(&end);

            if is_inv_start || is_inv_end {
                if is_inv_start {
                    must_panic(file_span!(), || string::str_from(CHAR_LENS, start)).unwrap();
                }
                if is_inv_end {
                    must_panic(file_span!(), || string::str_up_to(CHAR_LENS, end)).unwrap();
                }
                must_panic(file_span!(), || string::str_range(CHAR_LENS, start, end)).unwrap();
            } else {
                if start > end {
                    assert_eq!(string::str_range(CHAR_LENS, start, end), "");
                } else {
                    assert_eq!(
                        string::str_range(CHAR_LENS, start, end),
                        &CHAR_LENS[start..end]
                    );
                }
                assert_eq!(string::str_from(CHAR_LENS, start), &CHAR_LENS[start..]);
                assert_eq!(string::str_up_to(CHAR_LENS, end), &CHAR_LENS[..end]);
            }

            assert_eq!(
                string::get_range(CHAR_LENS, start, end),
                CHAR_LENS.get(start..end)
            );
            assert_eq!(string::get_from(CHAR_LENS, start), CHAR_LENS.get(start..));
            assert_eq!(string::get_up_to(CHAR_LENS, end), CHAR_LENS.get(..end));
        }
    }
}

fn get_valid_indices() -> Vec<usize> {
    CHAR_LENS
        .char_indices()
        .map(|x| x.0)
        .chain(std::iter::once(CHAR_LENS.len()))
        .collect()
}

#[test]
fn test_in_bounds() {
    let valid_indices = get_valid_indices();
    for start in valid_indices.iter().copied() {
        assert_eq!(string::get_from(CHAR_LENS, start), CHAR_LENS.get(start..));
        assert_eq!(string::str_from(CHAR_LENS, start), &CHAR_LENS[start..]);

        for end in valid_indices.iter().copied().skip_while(|e| *e < start) {
            assert_eq!(
                string::get_range(CHAR_LENS, start, end),
                CHAR_LENS.get(start..end)
            );
            assert_eq!(
                string::str_range(CHAR_LENS, start, end),
                &CHAR_LENS[start..end]
            );
        }
    }
    for end in valid_indices.iter().copied() {
        assert_eq!(string::get_up_to(CHAR_LENS, end), CHAR_LENS.get(..end));
        assert_eq!(string::str_up_to(CHAR_LENS, end), &CHAR_LENS[..end]);
    }
}

#[test]
fn test_out_of_bounds() {
    let valid_indices = get_valid_indices();

    for x in valid_indices.into_iter().chain(OOB_INDICES.iter().copied()) {
        assert_eq!(
            string::str_up_to(CHAR_LENS, x),
            &CHAR_LENS[..x.min(LEN)],
            "{}",
            x
        );
        assert_eq!(
            string::str_from(CHAR_LENS, x),
            &CHAR_LENS[x.min(LEN)..],
            "{}",
            x
        );
        assert_eq!(string::get_up_to(CHAR_LENS, x), CHAR_LENS.get(..x));
        assert_eq!(string::get_from(CHAR_LENS, x), CHAR_LENS.get(x..));

        for end in OOB_INDICES.iter().copied() {
            assert_eq!(
                string::str_range(CHAR_LENS, x, end),
                &CHAR_LENS[x.min(LEN)..end.min(LEN)]
            );
            assert_eq!(string::get_range(CHAR_LENS, x, end), None);
        }
    }
}
