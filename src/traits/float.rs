// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    num::FpCategory,
    ops::Neg
};

use super::{
    num::Num
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_float {
    ($($t:ty),* $(,)?) => {$(
        impl Float for $t {
            // ── Constants ───────────────────────────────────────────────────
            const MIN: Self = <$t>::MIN;

            const MAX: Self = <$t>::MAX;

            const EPSILON: Self = <$t>::EPSILON;

            const RADIANS_PER_DEGREE: Self = core::f64::consts::PI as $t / 180.0;

            // ── Functions ───────────────────────────────────────────────────
            #[inline]
            fn nan() -> Self { <$t>::NAN }

            #[inline]
            fn infinity() -> Self { <$t>::INFINITY }

            #[inline]
            fn neg_infinity() -> Self { <$t>::NEG_INFINITY }

            #[inline]
            fn neg_zero() -> Self { -0.0 }

            #[inline]
            fn min_value() -> Self { <$t>::MIN }

            #[inline]
            fn min_positive_value() -> Self { <$t>::MIN_POSITIVE }

            #[inline]
            fn max_value() -> Self { <$t>::MAX }

            #[inline]
            fn epsilon() -> Self { <$t>::EPSILON }

            // ── Methods ─────────────────────────────────────────────────────
            #[inline]
            fn is_nan(self) -> bool { <$t>::is_nan(self) }

            #[inline]
            fn is_infinite(self) -> bool { <$t>::is_infinite(self) }

            #[inline]
            fn is_finite(self) -> bool { <$t>::is_finite(self) }

            #[inline]
            fn is_normal(self) -> bool { <$t>::is_normal(self) }

            #[inline]
            fn is_subnormal(self) -> bool { <$t>::is_subnormal(self) }

            #[inline]
            fn classify(self) -> FpCategory { <$t>::classify(self) }

            #[inline]
            fn is_sign_positive(self) -> bool { <$t>::is_sign_positive(self) }

            #[inline]
            fn is_sign_negative(self) -> bool { <$t>::is_sign_negative(self) }

            #[inline]
            fn floor(self) -> Self { <$t>::floor(self) }

            #[inline]
            fn ceil(self) -> Self { <$t>::ceil(self) }

            #[inline]
            fn round(self) -> Self { <$t>::round(self) }

            #[inline]
            fn trunc(self) -> Self { <$t>::trunc(self) }

            #[inline]
            fn fract(self) -> Self { <$t>::fract(self) }

            #[inline]
            fn abs(self) -> Self { <$t>::abs(self) }

            #[inline]
            fn abs_sub(self, other: Self) -> Self {
                // `<$t>::abs_sub` is deprecated; this matches its semantics
                // (positive difference, NaN propagates).
                if self <= other { 0.0 } else { self - other }
            }

            #[inline]
            fn signum(self) -> Self { <$t>::signum(self) }

            #[inline]
            fn copysign(self, sign: Self) -> Self { <$t>::copysign(self, sign) }

            #[inline]
            fn max(self, other: Self) -> Self { <$t>::max(self, other) }

            #[inline]
            fn min(self, other: Self) -> Self { <$t>::min(self, other) }

            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self { <$t>::clamp(self, min, max) }

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self { <$t>::mul_add(self, a, b) }

            #[inline]
            fn recip(self) -> Self { <$t>::recip(self) }

            #[inline]
            fn powi(self, n: i32) -> Self { <$t>::powi(self, n) }

            #[inline]
            fn powf(self, n: Self) -> Self { <$t>::powf(self, n) }

            #[inline]
            fn sqrt(self) -> Self { <$t>::sqrt(self) }

            #[inline]
            fn cbrt(self) -> Self { <$t>::cbrt(self) }

            #[inline]
            fn hypot(self, other: Self) -> Self { <$t>::hypot(self, other) }

            #[inline]
            fn exp(self) -> Self { <$t>::exp(self) }

            #[inline]
            fn exp2(self) -> Self { <$t>::exp2(self) }

            #[inline]
            fn exp_m1(self) -> Self { <$t>::exp_m1(self) }

            #[inline]
            fn ln(self) -> Self { <$t>::ln(self) }

            #[inline]
            fn ln_1p(self) -> Self { <$t>::ln_1p(self) }

            #[inline]
            fn log(self, base: Self) -> Self { <$t>::log(self, base) }

            #[inline]
            fn log2(self) -> Self { <$t>::log2(self) }

            #[inline]
            fn log10(self) -> Self { <$t>::log10(self) }

            #[inline]
            fn sin(self) -> Self { <$t>::sin(self) }

            #[inline]
            fn cos(self) -> Self { <$t>::cos(self) }

            #[inline]
            fn tan(self) -> Self { <$t>::tan(self) }

            #[inline]
            fn asin(self) -> Self { <$t>::asin(self) }

            #[inline]
            fn acos(self) -> Self { <$t>::acos(self) }

            #[inline]
            fn atan(self) -> Self { <$t>::atan(self) }

            #[inline]
            fn atan2(self, other: Self) -> Self { <$t>::atan2(self, other) }

            #[inline]
            fn sin_cos(self) -> (Self, Self) { <$t>::sin_cos(self) }

            #[inline]
            fn to_degrees(self) -> Self { <$t>::to_degrees(self) }

            #[inline]
            fn to_radians(self) -> Self { <$t>::to_radians(self) }

            #[inline]
            fn sinh(self) -> Self { <$t>::sinh(self) }

            #[inline]
            fn cosh(self) -> Self { <$t>::cosh(self) }

            #[inline]
            fn tanh(self) -> Self { <$t>::tanh(self) }

            #[inline]
            fn asinh(self) -> Self { <$t>::asinh(self) }

            #[inline]
            fn acosh(self) -> Self { <$t>::acosh(self) }

            #[inline] fn atanh(self) -> Self { <$t>::atanh(self) }

            fn integer_decode(self) -> (u64, i16, i8) {
                const TOTAL_BITS: u32 = (core::mem::size_of::<$t>() * 8) as u32;
                const MAN_BITS: u32 = <$t>::MANTISSA_DIGITS - 1;
                const EXP_BITS: u32 = TOTAL_BITS - 1 - MAN_BITS;
                const BIAS: i16 = (<$t>::MAX_EXP - 1) as i16;

                const MAN_MASK: u64 = (1u64 << MAN_BITS) - 1;
                const EXP_MASK: u64 = (1u64 << EXP_BITS) - 1;

                let bits = self.to_bits() as u64;

                let sign: i8 = if bits >> (TOTAL_BITS - 1) == 0 { 1 } else { -1 };
                let raw_exp = ((bits >> MAN_BITS) & EXP_MASK) as i16;
                let mantissa = if raw_exp == 0 {
                    (bits & MAN_MASK) << 1
                } else {
                    (bits & MAN_MASK) | (1u64 << MAN_BITS)
                };

                (mantissa, raw_exp - BIAS - MAN_BITS as i16, sign)
            }
        }
    )*};
}

// ── `trait Float` Definition ────────────────────────────────────────────────
pub trait Float:
    Num
    + Copy
    + PartialOrd
    + Neg<Output = Self> {
    // ── Constants ───────────────────────────────────────────────────────────
    const MIN: Self;

    const MAX: Self;

    const EPSILON: Self;

    /// `π / 180`, used by the default `to_degrees` / `to_radians`.
    const RADIANS_PER_DEGREE: Self;

    // ── Functions ───────────────────────────────────────────────────────────
    fn nan() -> Self;

    fn infinity() -> Self;

    fn neg_infinity() -> Self;

    fn neg_zero() -> Self;

    fn min_value() -> Self;

    fn min_positive_value() -> Self;

    fn max_value() -> Self;

    #[inline]
    fn epsilon() -> Self { Self::EPSILON }

    // ── Methods ─────────────────────────────────────────────────────────────
    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    #[inline]
    fn is_subnormal(self) -> bool {
        self.classify() == FpCategory::Subnormal
    }

    fn classify(self) -> FpCategory;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn floor(self) -> Self;

    fn ceil(self) -> Self;

    fn round(self) -> Self;

    fn trunc(self) -> Self;

    fn fract(self) -> Self;

    fn abs(self) -> Self;

    fn abs_sub(self, other: Self) -> Self;

    fn signum(self) -> Self;

    #[inline]
    fn copysign(self, sign: Self) -> Self {
        if self.is_sign_negative() == sign.is_sign_negative() {
            self
        } else {
            self.neg()
        }
    }

    fn max(self, other: Self) -> Self;

    fn min(self, other: Self) -> Self;

    #[inline]
    fn clamp(self, min: Self, max: Self) -> Self {
        assert!(min <= max, "min > max, or either was NaN");

        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn mul_add(self, a: Self, b: Self) -> Self;

    fn recip(self) -> Self;

    fn powi(self, n: i32) -> Self;

    fn powf(self, n: Self) -> Self;

    fn sqrt(self) -> Self;

    fn cbrt(self) -> Self;

    fn hypot(self, other: Self) -> Self;

    fn exp(self) -> Self;

    fn exp2(self) -> Self;

    fn exp_m1(self) -> Self;

    fn ln(self) -> Self;

    fn ln_1p(self) -> Self;

    fn log(self, base: Self) -> Self;

    fn log2(self) -> Self;

    fn log10(self) -> Self;

    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn tan(self) -> Self;

    fn asin(self) -> Self;

    fn acos(self) -> Self;

    fn atan(self) -> Self;

    fn atan2(self, other: Self) -> Self;

    fn sin_cos(self) -> (Self, Self);

    #[inline]
    fn to_degrees(self) -> Self {
        self / Self::RADIANS_PER_DEGREE
    }

    #[inline]
    fn to_radians(self) -> Self {
        self * Self::RADIANS_PER_DEGREE
    }

    fn sinh(self) -> Self;

    fn cosh(self) -> Self;

    fn tanh(self) -> Self;

    fn asinh(self) -> Self;

    fn acosh(self) -> Self;

    fn atanh(self) -> Self;

    /// Returns `(mantissa, exponent, sign)` such that
    /// `sign * mantissa * 2^exponent` equals the original value.
    fn integer_decode(self) -> (u64, i16, i8);
}

// ── `Float` Implementations ─────────────────────────────────────────────────
impl_float!(
    f32,
    f64
);
