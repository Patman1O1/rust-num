// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    ops::{
        Neg
    }
};

use super::{
    int::Int,
    uint::Uint
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_iint {
    ($($T:ty => $P:ty),* $(,)?) => {$(
        // ── `Iint` for $T Implementations ───────────────────────────────────
        impl Iint for $T {
            // ── Types ───────────────────────────────────────────────────────
            type Unsigned = $P;

            // ── Methods ─────────────────────────────────────────────────────
            #[inline]
            fn abs(self) -> Self { <$T>::abs(self) }

            #[inline]
            fn abs_diff(self, other: Self) -> Self::Unsigned {
                <$T>::abs_diff(self, other)
            }

            #[inline]
            fn unsigned_abs(self) -> Self::Unsigned {
                <$T>::unsigned_abs(self)
            }

            #[inline]
            fn signum(self) -> Self { <$T>::signum(self) }

            #[inline]
            fn is_positive(self) -> bool { <$T>::is_positive(self) }

            #[inline]
            fn is_negative(self) -> bool { <$T>::is_negative(self) }

            #[inline]
            fn cast_unsigned(self) -> Self::Unsigned {
                <$T>::cast_unsigned(self)
            }

            #[inline]
            fn checked_abs(self) -> Option<Self> { <$T>::checked_abs(self) }

            #[inline]
            fn checked_isqrt(self) -> Option<Self> {
                <$T>::checked_isqrt(self)
            }

            #[inline]
            fn checked_add_unsigned(
                self,
                rhs: Self::Unsigned
            ) -> Option<Self> {
                <$T>::checked_add_unsigned(self, rhs)
            }

            #[inline]
            fn checked_sub_unsigned(
                self,
                rhs: Self::Unsigned
            ) -> Option<Self> {
                <$T>::checked_sub_unsigned(self, rhs)
            }

            #[inline]
            fn overflowing_abs(self) -> (Self, bool) {
                <$T>::overflowing_abs(self)
            }

            #[inline]
            fn overflowing_add_unsigned(
                self,
                rhs: Self::Unsigned
            ) -> (Self, bool) {
                <$T>::overflowing_add_unsigned(self, rhs)
            }

            #[inline]
            fn overflowing_sub_unsigned(
                self,
                rhs: Self::Unsigned
            ) -> (Self, bool) {
                <$T>::overflowing_sub_unsigned(self, rhs)
            }

            #[inline]
            fn saturating_abs(self) -> Self { <$T>::saturating_abs(self) }

            #[inline]
            fn saturating_neg(self) -> Self { <$T>::saturating_neg(self) }

            #[inline]
            fn saturating_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::saturating_add_unsigned(self, rhs)
            }

            #[inline]
            fn saturating_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::saturating_sub_unsigned(self, rhs)
            }

            #[inline]
            fn strict_abs(self) -> Self { <$T>::strict_abs(self) }

            #[inline]
            fn strict_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::strict_add_unsigned(self, rhs)
            }

            #[inline]
            fn strict_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::strict_sub_unsigned(self, rhs)
            }

            #[inline]
            unsafe fn unchecked_neg(self) -> Self {
                // SAFETY: forwarded; caller upholds
                // `<$T>::unchecked_neg`'s contract.
                unsafe { <$T>::unchecked_neg(self) }
            }

            #[inline]
            fn wrapping_abs(self) -> Self { <$T>::wrapping_abs(self) }

            #[inline]
            fn wrapping_add_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::wrapping_add_unsigned(self, rhs)
            }

            #[inline]
            fn wrapping_sub_unsigned(self, rhs: Self::Unsigned) -> Self {
                <$T>::wrapping_sub_unsigned(self, rhs)
            }
        }
    )*};
}

// ── `trait Iint` Definition ─────────────────────────────────────────────────
/// Signed primitive integers (`i8` … `i128`, `isize`).
pub trait Iint: Int + Neg<Output = Self> {
    // ── Types ───────────────────────────────────────────────────────────────
    type Unsigned: Uint<Signed = Self>;

    // ── Methods ─────────────────────────────────────────────────────────────
    fn abs(self) -> Self;

    fn abs_diff(self, other: Self) -> Self::Unsigned;

    fn unsigned_abs(self) -> Self::Unsigned;

    fn signum(self) -> Self;

    fn is_positive(self) -> bool;

    fn is_negative(self) -> bool;

    fn cast_unsigned(self) -> Self::Unsigned;

    fn checked_abs(self) -> Option<Self>;

    fn checked_isqrt(self) -> Option<Self>;

    fn checked_add_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;

    fn checked_sub_unsigned(self, rhs: Self::Unsigned) -> Option<Self>;

    fn overflowing_abs(self) -> (Self, bool);

    fn overflowing_add_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);

    fn overflowing_sub_unsigned(self, rhs: Self::Unsigned) -> (Self, bool);

    fn saturating_abs(self) -> Self;

    fn saturating_neg(self) -> Self;

    fn saturating_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    fn saturating_sub_unsigned(self, rhs: Self::Unsigned) -> Self;

    fn strict_abs(self) -> Self;

    fn strict_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    fn strict_sub_unsigned(self, rhs: Self::Unsigned) -> Self;

    /// # Safety
    /// `self` must not be `Self::MIN`.
    unsafe fn unchecked_neg(self) -> Self;

    fn wrapping_abs(self) -> Self;

    fn wrapping_add_unsigned(self, rhs: Self::Unsigned) -> Self;

    fn wrapping_sub_unsigned(self, rhs: Self::Unsigned) -> Self;
}

// ── `Iint` Implementations ──────────────────────────────────────────────────
impl_iint!(
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize
);
