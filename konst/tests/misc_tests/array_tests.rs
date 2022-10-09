#[test]
fn array_map_test() {
    use konst::array::map;

    {
        // ensuring that `return` returns from the enclosing named function.
        const fn map_evens<const N: usize>(input: [u8; N]) -> Option<[u8; N]> {
            Some(map!(input, |x| if x % 2 == 0 {
                x * 2
            } else {
                return None;
            }))
        }

        assert_eq!(map_evens([2, 4, 6]), Some([4, 8, 12]));
        assert_eq!(map_evens([2, 3, 6]), None);
    }
    {
        // ensuring that the type annotation is used
        const SQUARED: [u32; 3] = map!([3, 5, 8], |x: u32| x.pow(2));

        assert_eq!(SQUARED, [9, 25, 64]);
    }
    {
        // ensuring that functions can be used
        const X: [Option<u32>; 2] = map!([10, 20], Some);
        const Y: [u32; 3] = map!([0b1, 0b11, 0b111], <u32>::count_ones);

        assert_eq!(X, [Some(10), Some(20)]);
        assert_eq!(Y, [1, 2, 3]);
    }
}
