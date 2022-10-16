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
