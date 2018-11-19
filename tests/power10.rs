extern crate num_integer;
extern crate num_traits;

use num_integer::Power10;
use num_integer::floor_log10;
use num_traits::ToPrimitive;

fn compare_with_f64<T: Power10 + ToPrimitive>(x: T) {
    let expected = x.to_f64().unwrap().log10().floor() as u32;
    assert_eq!(x.floor_log10(), expected, "{}", x.to_f64().unwrap());
}

macro_rules! unsigned_power10 {
    ($T:ty, $I:ident) => {
        mod $I {
            use num_integer::Power10;
            use num_integer::is_power_of_ten;
            use num_integer::floor_log10;
            use super::compare_with_f64;

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

            #[test]
            fn test_floor_log10() {
                assert_eq!((0 as $T).floor_log10(), 0, "zero");
                compare_with_f64(<$T>::max_value());
                compare_with_f64(<$T>::max_value() - 1);
                compare_with_f64(<$T>::max_value() - 2);
                let mut x: $T = 1;
                let mut count: u32 = 0;
                let end: $T = <$T>::max_value() / 10;
                while x < end {
                    assert_eq!(floor_log10(x), count, "{}", x);
                    if x > 1 {
                        assert_eq!((x - 1).floor_log10(), count - 1, "{}", x);
                    }
                    assert_eq!((x + 1).floor_log10(), count, "{}", x);
                    x *= 10;
                    count += 1;
                }
                assert_eq!(x.floor_log10(), count, "{}", x);
                assert_eq!((x - 1).floor_log10(), count - 1, "{}", (x-1));
                assert_eq!((x + 1).floor_log10(), count, "{}", (x+1));

                //powers of 2
                x = 1;
                while x != 0 {
                    compare_with_f64(x);
                    compare_with_f64(x+1);
                    compare_with_f64(x-1);
                    x <<= 1;
                }
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

// this function generates the table used for floor_log10
// run with --release to avoid overflow
// it then has to be reversed (I used tac)
//#[test]
//fn print_digits() {
//    println!("({}, {}), ", 0, 1);
//    let mut x: u128 = 1;
//    let mut count: u32 = 0;
//    let mut pow10: u128 = 10;
//    while x != 0 {
//        print!("({}, {}), ", count, pow10);
//        let digits = (x as f64).log10().floor() as u32;
//        x <<= 1;
//        if x != 0 {
//            let next_digits = (x as f64).log10().floor() as u32;
//            if next_digits > digits {
//                count += 1;
//                pow10 *= 10;
//                println!();
//            }
//        }
//    }
//    println!("");
//}