use Integer;

/// Provides methods to compute functions related to powers of 10.
pub trait Power10: Integer {

    /// Returns `true` if the number is a power of 10.
    ///
    /// # Examples
    ///
    /// ~~~
    /// use num_integer::Power10;
    /// assert_eq!(100u32.is_power_of_ten(), true);
    /// assert_eq!(4u32.is_power_of_ten(), false);
    /// ~~~
    fn is_power_of_ten(&self) -> bool;
}

/// Returns `true` if the number is a power of 10.
#[inline]
pub fn is_power_of_ten<T: Power10>(x: T) -> bool {
    x.is_power_of_ten()
}

// implementation note: the is_power_of_ten algorithm is based on a perfect hash setup with very simple hash functions
// hash functions only use 32-bit operations for portable-speed
// this approach is about 20% faster than using leading_zeros() (which is used for fast log operations)
static POWER10_HASH_U16: [u32;8] = [1, 10, 10000, 0, 100, 1000, 0, 0];
static POWER10_HASH_U32: [u32;16] = [10000, 1, 10000000, 0, 100, 0, 100000, 100000000, 1000, 0, 10, 1000000000, 0, 1000000, 0, 0];
static POWER10_HASH_U64: [u64;32] = [1, 100000000000, 10000000000000000, 0, 1000000, 10, 0, 10000000000000, 10000000,
    0, 1000, 0, 0, 0, 1000000000000, 0, 1000000000000000, 0, 100000000000000,
    1000000000000000000, 100, 100000000, 100000000000000000, 0, 10000000000000000000,
    10000, 100000, 0, 1000000000, 0, 10000000000, 0];
#[cfg(has_i128)]
static POWER10_HASH_U128: [u128;64] = [100000000000000000000000000000000000, 0, 100000000000000000000000000000000,
    0, 1000, 0, 0, 0, 0,
    1000000000000000000000000000000000000, 1000000000, 0, 100000000000000, 100, 0, 0, 0, 100000, 0, 0,
    10000000000000, 100000000000, 10000000000000000000, 0, 0, 10000000000000000000000000000000000,
    100000000, 0, 1000000000000000000000000000000000, 1000000000000, 0, 100000000000000000000000000000000000000,
    10000000000000000, 100000000000000000000000000, 0, 10000000000000000000000000000000000000,
    1000000000000000000, 1, 10000000000000000000000000, 1000000000000000000000000, 100000000000000000000000000000,
    10000000, 10000000000000000000000000000, 0, 1000000000000000000000000000, 100000000000000000, 10000,
    0, 1000000, 1000000000000000000000000000000, 0, 100000000000000000000, 10, 0, 10000000000,
    10000000000000000000000, 0, 0, 10000000000000000000000000000000, 1000000000000000000000, 0,
    100000000000000000000000, 1000000000000000, 0];

// implementation note: reverse search is a bit faster than hash lookup for u8
#[inline]
fn is_pow10_u8(v: u8) -> bool {
    if v >= 100 { return v == 100; }
    if v >= 10 { return v == 10; }
    if v >= 1 { return v == 1; }
    false
}

// implementation note: at least on x86-64, 32bit ops are far faster than 16 bit ones, even ==
#[inline]
fn is_pow10_u16(v: u16) -> bool {
    v as u32 == POWER10_HASH_U16[((v as u32 >> 3) & 7) as usize]
}

#[inline]
fn is_pow10_u32(v: u32) -> bool {
    let hash = v ^ (v >> 14);
    v == POWER10_HASH_U32[(hash & 15) as usize]
}

#[inline]
fn is_pow10_u64(v: u64) -> bool {
    let mut hash: u32 = v as u32;
    hash ^= hash >> 3;
    hash = hash.rotate_right(1);
    hash ^= hash >> 23;
    v == POWER10_HASH_U64[(hash & 31) as usize]
}

#[cfg(has_i128)]
#[inline]
fn is_pow10_u128(v: u128) -> bool {
    let mut hash: u32 = v as u32 | (((v as u64) >> 32) as u32);
    hash = hash.wrapping_mul(1249991743).rotate_right(25);
    v == POWER10_HASH_U128[(hash & 63) as usize]
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn is_pow10_usize(v: usize) -> bool {
    is_pow10_u64(v as u64)
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn is_pow10_usize(v: usize) -> bool {
    is_pow10_u32(v as u32)
}

macro_rules! unsigned_power10 {
    ($T:ty, $pow_fn: ident) => {
        impl Power10 for $T {
            #[inline]
            fn is_power_of_ten(&self) -> bool {
                $pow_fn(*self)
            }
        }
    };
}

unsigned_power10!(u8, is_pow10_u8); //https://github.com/rust-lang/rust/issues/29599
unsigned_power10!(u16, is_pow10_u16);
unsigned_power10!(u32, is_pow10_u32);
unsigned_power10!(u64, is_pow10_u64);
#[cfg(has_i128)]
unsigned_power10!(u128, is_pow10_u128);
unsigned_power10!(usize, is_pow10_usize);





