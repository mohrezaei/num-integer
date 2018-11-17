extern crate num_integer;
extern crate num_traits;

macro_rules! unsigned_power10 {
    ($T:ty, $I:ident) => {
        mod $I {
            use num_integer::Power10;
            use num_integer::is_power_of_ten;

            #[test]
            fn test_pow_10() {
                assert_eq!((0 as $T).is_power_of_ten(), false);
                assert_eq!(<$T>::max_value().is_power_of_ten(), false);
                let mut x: $T = 1;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(is_power_of_ten(x), true);
                    assert_eq!((x - 1).is_power_of_ten(), false);
                    assert_eq!((x + 1).is_power_of_ten(), false);
                    x *= 10;
                }
                assert_eq!(x.is_power_of_ten(), true);
                assert_eq!((x - 1).is_power_of_ten(), false);
                assert_eq!((x + 1).is_power_of_ten(), false);
            }
        }
    };
}

unsigned_power10!(u8, u8);
unsigned_power10!(u16, u16);
unsigned_power10!(u32, u32);
unsigned_power10!(u64, u64);
#[cfg(has_i128)]
unsigned_power10!(u128, u128);
unsigned_power10!(usize, usize);

