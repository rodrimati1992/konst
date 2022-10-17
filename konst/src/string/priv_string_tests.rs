// index: 00 char: '🧡' len_utf8: 4
// index: 04 char: '🧠' len_utf8: 4
// index: 08 char: '₀' len_utf8: 3
// index: 11 char: '₁' len_utf8: 3
// index: 14 char: 'o' len_utf8: 1
// index: 15 char: 'ñ' len_utf8: 2
// index: 17 char: '个' len_utf8: 3
const B: &[u8] = "🧡🧠₀₁oñ个".as_bytes();

#[test]
#[should_panic]
fn invalid_start() {
    // SAFETY: this is a slice of a string
    unsafe { super::from_u8_subslice_of_str(&B[1..]) };
}

#[test]
#[should_panic]
fn invalid_end() {
    // SAFETY: this is a slice of a string
    unsafe { super::from_u8_subslice_of_str(&B[..B.len() - 1]) };
}

#[test]
#[should_panic]
fn invalid_both() {
    // SAFETY: this is a slice of a string
    unsafe { super::from_u8_subslice_of_str(&B[1..B.len() - 1]) };
}
