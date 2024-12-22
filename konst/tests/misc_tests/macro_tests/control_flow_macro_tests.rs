#[cfg(feature = "parsing")]
use konst::rebind_if_ok;

#[test]
fn for_range_break_continue() {
    {
        let mut vect = Vec::new();
        konst::for_range! {x in  0..usize::MAX =>
            if x == 5 {
                break;
            }
            vect.push(x);
        }
        assert_eq!(vect, vec![0, 1, 2, 3, 4]);
    }
    {
        let mut vect = Vec::new();
        konst::for_range! {x in  0..10 =>
            if x % 2 == 0 {
                continue;
            }
            vect.push(x);
        }
        assert_eq!(vect, vec![1, 3, 5, 7, 9]);
    }
}

#[cfg(feature = "parsing")]
#[test]
fn rebind_if_ok_test() {
    {
        let mut a = 10;
        let mut b = "hello".into();
        let res = Ok::<_, ()>((Default::default(), Default::default()));
        rebind_if_ok! {(a, b): (u32, &str) = res}
        assert_eq!(a, 0);
        assert_eq!(b, "");
    }
    {
        let mut b = 10;
        let res = Ok::<_, ()>((Default::default(), Default::default()));
        rebind_if_ok! {(_, b): (u32, u64) = res}
        assert_eq!(b, 0);
    }
    {
        let mut a = "hello".into();
        rebind_if_ok! {a: &str = Ok::<_, ()>(Default::default())}
        assert_eq!(a, "");
    }
    {
        rebind_if_ok! {_: &str = Ok::<_, ()>(Default::default())}
    }
}

#[test]
fn if_let_some_test() {
    const fn uses_macro<T>(opt: Option<T>) -> u32 {
        konst::if_let_Some!{x = opt => {
            core::mem::forget(x);
            3
        } else {
            5
        }}
    }

    assert_eq!(uses_macro(Some(8)), 3);
    assert_eq!(uses_macro(None::<String>), 5);
}


#[test]
fn while_let_some_test() {
    const fn uses_macro<T, const N: usize>(array: [T; N]) -> u32 {
        let mut iter = konst::array::ArrayConsumer::new(array);
        let mut ret = 0;
        konst::while_let_Some!{x = iter.next() => {
            core::mem::forget(x);
            ret += 2;
        }}
        iter.assert_is_empty();
        ret
    }

    assert_eq!(uses_macro([String::new(); 0]), 0);
    assert_eq!(uses_macro([3]), 2);
    assert_eq!(uses_macro([3, 5]), 4);
    assert_eq!(uses_macro([3, 5, 8]), 6);
    assert_eq!(uses_macro([3, 5, 8, 13]), 8);
}





