use super::{approx_log10, PanikVal, CAP};

type ArrString = arrayvec::ArrayString<CAP>;

fn get_numbers() -> impl Iterator<Item = usize> {
    (0..)
        .map_while(|pow| 10usize.checked_pow(pow))
        .chain((2..).map_while(|pow| 2usize.checked_pow(pow)))
        .chain([usize::MAX])
        .flat_map(|x| x.saturating_sub(2)..=x.saturating_add(2))
}

#[test]
fn approx_log10_test() {
    use core::fmt::Write;

    let mut buff = ArrString::new();

    for n in get_numbers() {
        buff.clear();
        write!(buff, "{n}").unwrap();

        let log10 = approx_log10(n);
        let len = buff.len() as u32;
        assert!(len <= log10, "{len} <= {log10}");
    }
}

#[test]
fn fmt_numbers() {
    for number in get_numbers() {
        for (prefix, suffix) in [
            ("", ""),
            ("-", ""),
            ("-", "-"),
            ("", "-"),
            ("prefix", "suffix"),
        ] {
            let actual = test_fmt_conc(&[
                PanikVal::Str(prefix),
                PanikVal::Usize(number),
                PanikVal::Str(suffix),
            ]);
            let expected = ArrString::try_from(format_args!("{prefix}{number}{suffix}")).unwrap();
            assert_eq!(actual, expected)
        }
    }
}

#[cfg(test)]
fn test_fmt_conc(slice: &[PanikVal<'_>]) -> ArrString {
    let arr = super::fmt_conc(slice);
    let mut ret = ArrString::new();

    core::str::from_utf8(&arr)
        .unwrap()
        .chars()
        .filter(|x| *x != '\0')
        .for_each(|c| ret.push(c));

    ret
}
