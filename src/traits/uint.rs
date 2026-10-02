// ── Aliases ─────────────────────────────────────────────────────────────────
use super::{
    int::Int,
    iint::Iint
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_uint {
    ($($T:ty => $P:ty),* $(,)?) => {$(
        // ── `Uint` for $T Implementations ───────────────────────────────────
        impl Uint for $T {
            // ── Types ───────────────────────────────────────────────────────
            type Signed = $P;

            // ── Methods ─────────────────────────────────────────────────────
            #[inline]
            fn abs_diff(self, other: Self) -> Self {
                <$T>::abs_diff(self, other)
            }

            #[inline]
            fn bit_width(self) -> u32 { <$T>::bit_width(self) }

            #[inline]
            fn is_power_of_two(self) -> bool { <$T>::is_power_of_two(self) }

            #[inline]
            fn next_power_of_two(self) -> Self {
                <$T>::next_power_of_two(self)
            }

            #[inline]
            fn checked_next_power_of_two(self) -> Option<Self> {
                <$T>::checked_next_power_of_two(self)
            }

            #[inline]
            fn div_ceil(self, rhs: Self) -> Self { <$T>::div_ceil(self, rhs) }

            #[inline]
            fn next_multiple_of(self, rhs: Self) -> Self {
                <$T>::next_multiple_of(self, rhs)
            }

            #[inline]
            fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
                <$T>::checked_next_multiple_of(self, rhs)
            }

            #[inline]
            fn is_multiple_of(self, rhs: Self) -> bool {
                <$T>::is_multiple_of(self, rhs)
            }

            #[inline]
            fn cast_signed(self) -> Self::Signed { <$T>::cast_signed(self) }

            #[inline]
            fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self> {
                <$T>::checked_add_signed(self, rhs)
            }

            #[inline]
            fn checked_sub_signed(self, rhs: Self::Signed) -> Option<Self> {
                <$T>::checked_sub_signed(self, rhs)
            }

            #[inline]
            fn checked_signed_diff(self, rhs: Self) -> Option<Self::Signed> {
                <$T>::checked_signed_diff(self, rhs)
            }

            #[inline]
            fn overflowing_add_signed(
                self,
                rhs: Self::Signed
            ) -> (Self, bool) {
                <$T>::overflowing_add_signed(self, rhs)
            }

            #[inline]
            fn overflowing_sub_signed(
                self,
                rhs: Self::Signed
            ) -> (Self, bool) {
                <$T>::overflowing_sub_signed(self, rhs)
            }

            #[inline]
            fn saturating_add_signed(self, rhs: Self::Signed) -> Self {
                <$T>::saturating_add_signed(self, rhs)
            }

            #[inline]
            fn saturating_sub_signed(self, rhs: Self::Signed) -> Self {
                <$T>::saturating_sub_signed(self, rhs)
            }

            #[inline]
            fn strict_add_signed(self, rhs: Self::Signed) -> Self {
                <$T>::strict_add_signed(self, rhs)
            }

            #[inline]
            fn strict_sub_signed(self, rhs: Self::Signed) -> Self {
                <$T>::strict_sub_signed(self, rhs)
            }

            #[inline]
            fn wrapping_add_signed(self, rhs: Self::Signed) -> Self {
                <$T>::wrapping_add_signed(self, rhs)
            }

            #[inline]
            fn wrapping_sub_signed(self, rhs: Self::Signed) -> Self {
                <$T>::wrapping_sub_signed(self, rhs)
            }

            #[inline]
            fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
                <$T>::carrying_add(self, rhs, carry)
            }

            #[inline]
            fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
                <$T>::borrowing_sub(self, rhs, borrow)
            }

            #[inline]
            fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self) {
                <$T>::carrying_mul(self, rhs, carry)
            }

            #[inline]
            fn carrying_mul_add(
                self,
                rhs: Self,
                carry: Self,
                add: Self
            ) -> (Self, Self) {
                <$T>::carrying_mul_add(self, rhs, carry, add)
            }
        }
    )*};
}

// ── `trait Uint` Definition ─────────────────────────────────────────────────
/// Unsigned primitive integers (`u8` … `u128`, `usize`).
pub trait Uint: Int {
    // ── Types ───────────────────────────────────────────────────────────────
    type Signed: Iint<Unsigned = Self>;

    // ── Methods ─────────────────────────────────────────────────────────────
    fn abs_diff(self, other: Self) -> Self;

    fn bit_width(self) -> u32;

    fn is_power_of_two(self) -> bool;

    fn next_power_of_two(self) -> Self;

    fn checked_next_power_of_two(self) -> Option<Self>;

    fn div_ceil(self, rhs: Self) -> Self;

    fn next_multiple_of(self, rhs: Self) -> Self;

    fn checked_next_multiple_of(self, rhs: Self) -> Option<Self>;

    fn is_multiple_of(self, rhs: Self) -> bool;

    fn cast_signed(self) -> Self::Signed;

    fn checked_add_signed(self, rhs: Self::Signed) -> Option<Self>;

    fn checked_sub_signed(self, rhs: Self::Signed) -> Option<Self>;

    fn checked_signed_diff(self, rhs: Self) -> Option<Self::Signed>;

    fn overflowing_add_signed(self, rhs: Self::Signed) -> (Self, bool);

    fn overflowing_sub_signed(self, rhs: Self::Signed) -> (Self, bool);

    fn saturating_add_signed(self, rhs: Self::Signed) -> Self;

    fn saturating_sub_signed(self, rhs: Self::Signed) -> Self;

    fn strict_add_signed(self, rhs: Self::Signed) -> Self;

    fn strict_sub_signed(self, rhs: Self::Signed) -> Self;

    fn wrapping_add_signed(self, rhs: Self::Signed) -> Self;

    fn wrapping_sub_signed(self, rhs: Self::Signed) -> Self;

    fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool);

    fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool);

    fn carrying_mul(self, rhs: Self, carry: Self) -> (Self, Self);

    fn carrying_mul_add(
        self,
        rhs: Self,
        carry: Self,
        add: Self
    ) -> (Self, Self);
}

// ── `Uint` Implementations ──────────────────────────────────────────────────
impl_uint!(
    u8 => i8,
    u16 => i16,
    u32 => i32,
    u64 => i64,
    u128 => i128,
    usize => isize
);
