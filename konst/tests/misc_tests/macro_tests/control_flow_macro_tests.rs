use konst::{option, slice};

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

#[test]
fn if_let_some_test() {
    const fn uses_macro<T>(opt: Option<T>) -> u32 {
        konst::if_let_Some! {x = opt => {
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
    const fn uses_macro<T, const N: usize>(mut array: [Option<T>; N]) -> u32 {
        let mut i = 0;
        let mut ret = 0;
        konst::while_let_Some! {x =
            option::and_then!(slice::get_mut(&mut array, i), |x| x.take())
        => {
            core::mem::forget(x);
            ret += 2;

            i += 1;
        }}
        core::mem::forget(array);
        ret
    }

    assert_eq!(uses_macro([String::new(); 0].map(Some)), 0);
    assert_eq!(uses_macro([3].map(Some)), 2);
    assert_eq!(uses_macro([3, 5].map(Some)), 4);
    assert_eq!(uses_macro([3, 5, 8].map(Some)), 6);
    assert_eq!(uses_macro([3, 5, 8, 13].map(Some)), 8);
}
