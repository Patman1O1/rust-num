// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    num::ParseIntError
};

use super::{
    int_ops::IntOps
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_int {
    ($($t:ty),* $(,)?) => {$(
        // ── `Int` for $t Implementations ────────────────────────────────────
        impl Int for $t {
            // ── Types ───────────────────────────────────────────────────────
            type Bytes = [u8; core::mem::size_of::<$t>()];

            // ── Constants ───────────────────────────────────────────────────
            const MIN: Self = <$t>::MIN;

            const MAX: Self = <$t>::MAX;

            const BITS: u32 = <$t>::BITS;

            // ── Functions ───────────────────────────────────────────────────
            #[inline]
            fn min_value() -> Self { <$t>::MIN }

            #[inline]
            fn max_value() -> Self { <$t>::MAX }

            #[inline]
            fn from_be(x: Self) -> Self { <$t>::from_be(x) }

            #[inline]
            fn from_le(x: Self) -> Self { <$t>::from_le(x) }

            #[inline]
            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_be_bytes(bytes)
            }

            #[inline]
            fn from_le_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_le_bytes(bytes)
            }

            #[inline]
            fn from_ne_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_ne_bytes(bytes)
            }

            #[inline]
            fn from_str_radix(
                src: &str,
                radix: u32
            ) -> Result<Self, ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }

            // ── Methods ─────────────────────────────────────────────────────
            #[inline]
            fn count_ones(self) -> u32 { <$t>::count_ones(self) }

            #[inline]
            fn count_zeros(self) -> u32 { <$t>::count_zeros(self) }

            #[inline]
            fn leading_ones(self) -> u32 { <$t>::leading_ones(self) }

            #[inline]
            fn leading_zeros(self) -> u32 { <$t>::leading_zeros(self) }

            #[inline]
            fn trailing_ones(self) -> u32 { <$t>::trailing_ones(self) }

            #[inline]
            fn trailing_zeros(self) -> u32 { <$t>::trailing_zeros(self) }

            #[inline]
            fn highest_one(self) -> Option<u32> { <$t>::highest_one(self) }

            #[inline]
            fn lowest_one(self) -> Option<u32> { <$t>::lowest_one(self) }

            #[inline]
            fn isolate_highest_one(self) -> Self {
                <$t>::isolate_highest_one(self)
            }

            #[inline]
            fn isolate_lowest_one(self) -> Self {
                <$t>::isolate_lowest_one(self)
            }

            #[inline]
            fn reverse_bits(self) -> Self { <$t>::reverse_bits(self) }

            #[inline]
            fn rotate_left(self, n: u32) -> Self { <$t>::rotate_left(self, n) }

            #[inline]
            fn rotate_right(self, n: u32) -> Self {
                <$t>::rotate_right(self, n)
            }

            #[inline]
            fn swap_bytes(self) -> Self { <$t>::swap_bytes(self) }

            #[inline]
            fn unbounded_shl(self, rhs: u32) -> Self {
                <$t>::unbounded_shl(self, rhs)
            }

            #[inline]
            fn unbounded_shr(self, rhs: u32) -> Self {
                <$t>::unbounded_shr(self, rhs)
            }

            #[inline]
            fn to_be(self) -> Self { <$t>::to_be(self) }

            #[inline]
            fn to_le(self) -> Self { <$t>::to_le(self) }

            #[inline]
            fn to_be_bytes(self) -> Self::Bytes { <$t>::to_be_bytes(self) }

            #[inline]
            fn to_le_bytes(self) -> Self::Bytes { <$t>::to_le_bytes(self) }

            #[inline]
            fn to_ne_bytes(self) -> Self::Bytes { <$t>::to_ne_bytes(self) }

            #[inline]
            fn pow(self, exp: u32) -> Self { <$t>::pow(self, exp) }

            #[inline]
            fn isqrt(self) -> Self { <$t>::isqrt(self) }

            #[inline]
            fn ilog(self, base: Self) -> u32 { <$t>::ilog(self, base) }

            #[inline]
            fn ilog2(self) -> u32 { <$t>::ilog2(self) }

            #[inline]
            fn ilog10(self) -> u32 { <$t>::ilog10(self) }

            #[inline]
            fn div_euclid(self, rhs: Self) -> Self {
                <$t>::div_euclid(self, rhs)
            }

            #[inline]
            fn rem_euclid(self, rhs: Self) -> Self {
                <$t>::rem_euclid(self, rhs)
            }

            #[inline]
            fn midpoint(self, rhs: Self) -> Self { <$t>::midpoint(self, rhs) }

            #[inline]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                <$t>::checked_add(self, rhs)
            }

            #[inline]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                <$t>::checked_sub(self, rhs)
            }

            #[inline]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                <$t>::checked_mul(self, rhs)
            }

            #[inline]
            fn checked_div(self, rhs: Self) -> Option<Self> {
                <$t>::checked_div(self, rhs)
            }

            #[inline]
            fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                <$t>::checked_div_euclid(self, rhs)
            }

            #[inline]
            fn checked_rem(self, rhs: Self) -> Option<Self> {
                <$t>::checked_rem(self, rhs)
            }

            #[inline]
            fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                <$t>::checked_rem_euclid(self, rhs)
            }

            #[inline]
            fn checked_neg(self) -> Option<Self> { <$t>::checked_neg(self) }

            #[inline]
            fn checked_pow(self, exp: u32) -> Option<Self> {
                <$t>::checked_pow(self, exp)
            }

            #[inline]
            fn checked_shl(self, rhs: u32) -> Option<Self> {
                <$t>::checked_shl(self, rhs)
            }

            #[inline]
            fn checked_shr(self, rhs: u32) -> Option<Self> {
                <$t>::checked_shr(self, rhs)
            }

            #[inline]
            fn checked_ilog(self, base: Self) -> Option<u32> {
                <$t>::checked_ilog(self, base)
            }

            #[inline]
            fn checked_ilog2(self) -> Option<u32> { <$t>::checked_ilog2(self) }

            #[inline]
            fn checked_ilog10(self) -> Option<u32> {
                <$t>::checked_ilog10(self)
            }

            #[inline]
            fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_add(self, rhs)
            }

            #[inline]
            fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_sub(self, rhs)
            }

            #[inline]
            fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_mul(self, rhs)
            }

            #[inline]
            fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_div(self, rhs)
            }

            #[inline]
            fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_div_euclid(self, rhs)
            }

            #[inline]
            fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_rem(self, rhs)
            }

            #[inline]
            fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                <$t>::overflowing_rem_euclid(self, rhs)
            }

            #[inline]
            fn overflowing_neg(self) -> (Self, bool) {
                <$t>::overflowing_neg(self)
            }

            #[inline]
            fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                <$t>::overflowing_pow(self, exp)
            }

            #[inline]
            fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                <$t>::overflowing_shl(self, rhs)
            }

            #[inline]
            fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                <$t>::overflowing_shr(self, rhs)
            }

            #[inline]
            fn saturating_add(self, rhs: Self) -> Self {
                <$t>::saturating_add(self, rhs)
            }

            #[inline]
            fn saturating_sub(self, rhs: Self) -> Self {
                <$t>::saturating_sub(self, rhs)
            }

            #[inline]
            fn saturating_mul(self, rhs: Self) -> Self {
                <$t>::saturating_mul(self, rhs)
            }

            #[inline]
            fn saturating_div(self, rhs: Self) -> Self {
                <$t>::saturating_div(self, rhs)
            }

            #[inline]
            fn saturating_pow(self, exp: u32) -> Self {
                <$t>::saturating_pow(self, exp)
            }

            #[inline]
            fn strict_add(self, rhs: Self) -> Self {
                <$t>::strict_add(self, rhs)
            }

            #[inline]
            fn strict_sub(self, rhs: Self) -> Self {
                <$t>::strict_sub(self, rhs)
            }

            #[inline]
            fn strict_mul(self, rhs: Self) -> Self {
                <$t>::strict_mul(self, rhs)
            }

            #[inline]
            fn strict_div(self, rhs: Self) -> Self {
                <$t>::strict_div(self, rhs)
            }

            #[inline]
            fn strict_div_euclid(self, rhs: Self) -> Self {
                <$t>::strict_div_euclid(self, rhs)
            }

            #[inline]
            fn strict_rem(self, rhs: Self) -> Self {
                <$t>::strict_rem(self, rhs)
            }

            #[inline]
            fn strict_rem_euclid(self, rhs: Self) -> Self {
                <$t>::strict_rem_euclid(self, rhs)
            }

            #[inline]
            fn strict_neg(self) -> Self { <$t>::strict_neg(self) }

            #[inline]
            fn strict_pow(self, exp: u32) -> Self {
                <$t>::strict_pow(self, exp)
            }

            #[inline]
            fn strict_shl(self, rhs: u32) -> Self {
                <$t>::strict_shl(self, rhs)
            }

            #[inline]
            fn strict_shr(self, rhs: u32) -> Self {
                <$t>::strict_shr(self, rhs)
            }

            #[inline]
            unsafe fn unchecked_add(self, rhs: Self) -> Self {
                // SAFETY: forwarded; caller upholds 
                // `<$t>::unchecked_add`'s contract.
                unsafe { <$t>::unchecked_add(self, rhs) }
            }

            #[inline]
            unsafe fn unchecked_sub(self, rhs: Self) -> Self {
                // SAFETY: forwarded; caller upholds 
                // `<$t>::unchecked_sub`'s contract.
                unsafe { <$t>::unchecked_sub(self, rhs) }
            }

            #[inline]
            unsafe fn unchecked_mul(self, rhs: Self) -> Self {
                // SAFETY: forwarded; caller upholds 
                // `<$t>::unchecked_mul`'s contract.
                unsafe { <$t>::unchecked_mul(self, rhs) }
            }

            #[inline]
            unsafe fn unchecked_shl(self, rhs: u32) -> Self {
                // SAFETY: forwarded; caller upholds 
                // `<$t>::unchecked_shl`'s contract.
                unsafe { <$t>::unchecked_shl(self, rhs) }
            }

            #[inline]
            unsafe fn unchecked_shr(self, rhs: u32) -> Self {
                // SAFETY: forwarded; caller upholds
                // `<$t>::unchecked_shr`'s contract.
                unsafe { <$t>::unchecked_shr(self, rhs) }
            }

            #[inline]
            fn wrapping_add(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }

            #[inline]
            fn wrapping_sub(self, rhs: Self) -> Self {
                <$t>::wrapping_sub(self, rhs)
            }

            #[inline]
            fn wrapping_mul(self, rhs: Self) -> Self {
                <$t>::wrapping_mul(self, rhs)
            }

            #[inline]
            fn wrapping_div(self, rhs: Self) -> Self {
                <$t>::wrapping_div(self, rhs)
            }

            #[inline]
            fn wrapping_div_euclid(self, rhs: Self) -> Self {
                <$t>::wrapping_div_euclid(self, rhs)
            }

            #[inline]
            fn wrapping_rem(self, rhs: Self) -> Self {
                <$t>::wrapping_rem(self, rhs)
            }

            #[inline]
            fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                <$t>::wrapping_rem_euclid(self, rhs)
            }

            #[inline]
            fn wrapping_neg(self) -> Self { <$t>::wrapping_neg(self) }

            #[inline]
            fn wrapping_pow(self, exp: u32) -> Self {
                <$t>::wrapping_pow(self, exp)
            }

            #[inline]
            fn wrapping_shl(self, rhs: u32) -> Self {
                <$t>::wrapping_shl(self, rhs)
            }

            #[inline]
            fn wrapping_shr(self, rhs: u32) -> Self {
                <$t>::wrapping_shr(self, rhs)
            }
        }
    )*};
}

// ── `trait Int` Definition ──────────────────────────────────────────────────
pub trait Int:
    Sized
    + Copy
    + Eq
    + Ord
    + PartialOrd
    + IntOps {
    // ── Types ───────────────────────────────────────────────────────────────
    type Bytes: Copy + Default + AsRef<[u8]> + AsMut<[u8]>;

    // ── Constants ───────────────────────────────────────────────────────────
    const MIN: Self;

    const MAX: Self;

    const BITS: u32;

    // ── Functions ───────────────────────────────────────────────────────────
    fn min_value() -> Self;

    fn max_value() -> Self;

    fn from_be(x: Self) -> Self;

    fn from_le(x: Self) -> Self;

    fn from_be_bytes(bytes: Self::Bytes) -> Self;

    fn from_le_bytes(bytes: Self::Bytes) -> Self;

    fn from_ne_bytes(bytes: Self::Bytes) -> Self;

    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;

    // ── Methods ─────────────────────────────────────────────────────────────
    fn count_ones(self) -> u32;

    fn count_zeros(self) -> u32;

    fn leading_ones(self) -> u32;

    fn leading_zeros(self) -> u32;

    fn trailing_ones(self) -> u32;

    fn trailing_zeros(self) -> u32;

    fn highest_one(self) -> Option<u32>;

    fn lowest_one(self) -> Option<u32>;

    fn isolate_highest_one(self) -> Self;

    fn isolate_lowest_one(self) -> Self;

    fn reverse_bits(self) -> Self;

    fn rotate_left(self, n: u32) -> Self;

    fn rotate_right(self, n: u32) -> Self;

    fn swap_bytes(self) -> Self;

    fn unbounded_shl(self, rhs: u32) -> Self;

    fn unbounded_shr(self, rhs: u32) -> Self;

    fn to_be(self) -> Self;

    fn to_le(self) -> Self;

    fn to_be_bytes(self) -> Self::Bytes;

    fn to_le_bytes(self) -> Self::Bytes;

    fn to_ne_bytes(self) -> Self::Bytes;

    fn pow(self, exp: u32) -> Self;

    fn isqrt(self) -> Self;

    fn ilog(self, base: Self) -> u32;

    fn ilog2(self) -> u32;

    fn ilog10(self) -> u32;

    fn div_euclid(self, rhs: Self) -> Self;

    fn rem_euclid(self, rhs: Self) -> Self;

    fn midpoint(self, rhs: Self) -> Self;

    fn checked_add(self, rhs: Self) -> Option<Self>;

    fn checked_sub(self, rhs: Self) -> Option<Self>;

    fn checked_mul(self, rhs: Self) -> Option<Self>;

    fn checked_div(self, rhs: Self) -> Option<Self>;

    fn checked_div_euclid(self, rhs: Self) -> Option<Self>;

    fn checked_rem(self, rhs: Self) -> Option<Self>;

    fn checked_rem_euclid(self, rhs: Self) -> Option<Self>;

    fn checked_neg(self) -> Option<Self>;

    fn checked_pow(self, exp: u32) -> Option<Self>;

    fn checked_shl(self, rhs: u32) -> Option<Self>;

    fn checked_shr(self, rhs: u32) -> Option<Self>;

    fn checked_ilog(self, base: Self) -> Option<u32>;

    fn checked_ilog2(self) -> Option<u32>;

    fn checked_ilog10(self) -> Option<u32>;

    fn overflowing_add(self, rhs: Self) -> (Self, bool);

    fn overflowing_sub(self, rhs: Self) -> (Self, bool);

    fn overflowing_mul(self, rhs: Self) -> (Self, bool);

    fn overflowing_div(self, rhs: Self) -> (Self, bool);

    fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool);

    fn overflowing_rem(self, rhs: Self) -> (Self, bool);

    fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool);

    fn overflowing_neg(self) -> (Self, bool);

    fn overflowing_pow(self, exp: u32) -> (Self, bool);

    fn overflowing_shl(self, rhs: u32) -> (Self, bool);

    fn overflowing_shr(self, rhs: u32) -> (Self, bool);

    fn saturating_add(self, rhs: Self) -> Self;

    fn saturating_sub(self, rhs: Self) -> Self;

    fn saturating_mul(self, rhs: Self) -> Self;

    fn saturating_div(self, rhs: Self) -> Self;

    fn saturating_pow(self, exp: u32) -> Self;

    fn strict_add(self, rhs: Self) -> Self;

    fn strict_sub(self, rhs: Self) -> Self;

    fn strict_mul(self, rhs: Self) -> Self;

    fn strict_div(self, rhs: Self) -> Self;

    fn strict_div_euclid(self, rhs: Self) -> Self;

    fn strict_rem(self, rhs: Self) -> Self;

    fn strict_rem_euclid(self, rhs: Self) -> Self;

    fn strict_neg(self) -> Self;

    fn strict_pow(self, exp: u32) -> Self;

    fn strict_shl(self, rhs: u32) -> Self;

    fn strict_shr(self, rhs: u32) -> Self;

    /// # Safety
    /// `self + rhs` must not overflow.
    unsafe fn unchecked_add(self, rhs: Self) -> Self;

    /// # Safety
    /// `self - rhs` must not overflow.
    unsafe fn unchecked_sub(self, rhs: Self) -> Self;

    /// # Safety
    /// `self * rhs` must not overflow.
    unsafe fn unchecked_mul(self, rhs: Self) -> Self;

    /// # Safety
    /// `rhs` must be less than `Self::BITS`.
    unsafe fn unchecked_shl(self, rhs: u32) -> Self;

    /// # Safety
    /// `rhs` must be less than `Self::BITS`.
    unsafe fn unchecked_shr(self, rhs: u32) -> Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    fn wrapping_sub(self, rhs: Self) -> Self;

    fn wrapping_mul(self, rhs: Self) -> Self;

    fn wrapping_div(self, rhs: Self) -> Self;

    fn wrapping_div_euclid(self, rhs: Self) -> Self;

    fn wrapping_rem(self, rhs: Self) -> Self;

    fn wrapping_rem_euclid(self, rhs: Self) -> Self;

    fn wrapping_neg(self) -> Self;

    fn wrapping_pow(self, exp: u32) -> Self;

    fn wrapping_shl(self, rhs: u32) -> Self;

    fn wrapping_shr(self, rhs: u32) -> Self;
}

// ── `Int` Implementations ───────────────────────────────────────────────────
impl_int!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize
);
