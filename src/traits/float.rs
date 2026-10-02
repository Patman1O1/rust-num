// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    f64,
    mem,
    num::FpCategory,
    ops::Neg
};

use super::{
    real::Real
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_float {
    ($($T:ty),* $(,)?) => {$(
        impl Float for $T {
            // ── Constants ───────────────────────────────────────────────────
            const EPSILON: Self = <$T>::EPSILON;

            const RADIANS_PER_DEGREE: Self = f64::consts::PI as $T / 180.0;

            // ── Functions ───────────────────────────────────────────────────
            #[inline]
            fn nan() -> Self { <$T>::NAN }

            #[inline]
            fn infinity() -> Self { <$T>::INFINITY }

            #[inline]
            fn neg_infinity() -> Self { <$T>::NEG_INFINITY }

            #[inline]
            fn neg_zero() -> Self { -0.0 }

            #[inline]
            fn min_value() -> Self { <$T>::MIN }

            #[inline]
            fn min_positive_value() -> Self { <$T>::MIN_POSITIVE }

            #[inline]
            fn max_value() -> Self { <$T>::MAX }

            #[inline]
            fn epsilon() -> Self { <$T>::EPSILON }

            // ── Methods ─────────────────────────────────────────────────────
            #[inline]
            fn is_nan(self) -> bool { <$T>::is_nan(self) }

            #[inline]
            fn is_infinite(self) -> bool { <$T>::is_infinite(self) }

            #[inline]
            fn is_finite(self) -> bool { <$T>::is_finite(self) }

            #[inline]
            fn is_normal(self) -> bool { <$T>::is_normal(self) }

            #[inline]
            fn is_subnormal(self) -> bool { <$T>::is_subnormal(self) }

            #[inline]
            fn classify(self) -> FpCategory { <$T>::classify(self) }

            #[inline]
            fn is_sign_positive(self) -> bool { <$T>::is_sign_positive(self) }

            #[inline]
            fn is_sign_negative(self) -> bool { <$T>::is_sign_negative(self) }

            #[inline]
            fn floor(self) -> Self { <$T>::floor(self) }

            #[inline]
            fn ceil(self) -> Self { <$T>::ceil(self) }

            #[inline]
            fn round(self) -> Self { <$T>::round(self) }

            #[inline]
            fn trunc(self) -> Self { <$T>::trunc(self) }

            #[inline]
            fn fract(self) -> Self { <$T>::fract(self) }

            #[inline]
            fn abs(self) -> Self { <$T>::abs(self) }

            #[inline]
            fn abs_sub(self, other: Self) -> Self {
                // `<$T>::abs_sub` is deprecated; this matches its semantics
                // (positive difference, NaN propagates).
                if self <= other {
                    0.0
                } else {
                    self - other
                }
            }

            #[inline]
            fn signum(self) -> Self { <$T>::signum(self) }

            #[inline]
            fn copysign(self, sign: Self) -> Self {
                <$T>::copysign(self, sign)
            }

            #[inline]
            fn max(self, other: Self) -> Self { <$T>::max(self, other) }

            #[inline]
            fn min(self, other: Self) -> Self { <$T>::min(self, other) }

            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                <$T>::clamp(self, min, max)
            }

            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                <$T>::mul_add(self, a, b)
            }

            #[inline]
            fn recip(self) -> Self { <$T>::recip(self) }

            #[inline]
            fn powi(self, n: i32) -> Self { <$T>::powi(self, n) }

            #[inline]
            fn powf(self, n: Self) -> Self { <$T>::powf(self, n) }

            #[inline]
            fn sqrt(self) -> Self { <$T>::sqrt(self) }

            #[inline]
            fn cbrt(self) -> Self { <$T>::cbrt(self) }

            #[inline]
            fn hypot(self, other: Self) -> Self { <$T>::hypot(self, other) }

            #[inline]
            fn exp(self) -> Self { <$T>::exp(self) }

            #[inline]
            fn exp2(self) -> Self { <$T>::exp2(self) }

            #[inline]
            fn exp_m1(self) -> Self { <$T>::exp_m1(self) }

            #[inline]
            fn ln(self) -> Self { <$T>::ln(self) }

            #[inline]
            fn ln_1p(self) -> Self { <$T>::ln_1p(self) }

            #[inline]
            fn log(self, base: Self) -> Self { <$T>::log(self, base) }

            #[inline]
            fn log2(self) -> Self { <$T>::log2(self) }

            #[inline]
            fn log10(self) -> Self { <$T>::log10(self) }

            #[inline]
            fn sin(self) -> Self { <$T>::sin(self) }

            #[inline]
            fn cos(self) -> Self { <$T>::cos(self) }

            #[inline]
            fn tan(self) -> Self { <$T>::tan(self) }

            #[inline]
            fn asin(self) -> Self { <$T>::asin(self) }

            #[inline]
            fn acos(self) -> Self { <$T>::acos(self) }

            #[inline]
            fn atan(self) -> Self { <$T>::atan(self) }

            #[inline]
            fn atan2(self, other: Self) -> Self { <$T>::atan2(self, other) }

            #[inline]
            fn sin_cos(self) -> (Self, Self) { <$T>::sin_cos(self) }

            #[inline]
            fn to_degrees(self) -> Self { <$T>::to_degrees(self) }

            #[inline]
            fn to_radians(self) -> Self { <$T>::to_radians(self) }

            #[inline]
            fn sinh(self) -> Self { <$T>::sinh(self) }

            #[inline]
            fn cosh(self) -> Self { <$T>::cosh(self) }

            #[inline]
            fn tanh(self) -> Self { <$T>::tanh(self) }

            #[inline]
            fn asinh(self) -> Self { <$T>::asinh(self) }

            #[inline]
            fn acosh(self) -> Self { <$T>::acosh(self) }

            #[inline] fn atanh(self) -> Self { <$T>::atanh(self) }

            fn integer_decode(self) -> (u64, i16, i8) {
                const TOTAL_BITS: u32 = (mem::size_of::<$T>() * 8) as u32;
                const MAN_BITS: u32 = <$T>::MANTISSA_DIGITS - 1;
                const EXP_BITS: u32 = TOTAL_BITS - 1 - MAN_BITS;
                const BIAS: i16 = (<$T>::MAX_EXP - 1) as i16;

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
    Real
    + Copy
    + PartialOrd
    + Neg<Output = Self> {
    // ── Constants ───────────────────────────────────────────────────────────
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
