// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    clone::Clone,
    cmp::PartialEq,
    default::Default,
    iter::{Product, Sum},
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Rem,
        RemAssign,
        Sub,
        SubAssign
    }
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_num {
    (int: $($I:ty),*; float: $($F:ty),*) => {
        // ── `Num for $I` Implementation ─────────────────────────────────────
        $(impl Num for $I {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        })*

        // ── `Num for $F` Implementations ────────────────────────────────────
        $(impl Num for $F {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
        })*
    };
}

// ── `trait Num` Definition ──────────────────────────────────────────────────
pub trait Num:
    Copy + Clone + Default + PartialEq + Product + Sum
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign {
        // ── Constants ───────────────────────────────────────────────────────
        const ZERO: Self;
        const ONE: Self;
    }

// ── `Num` Implementations ───────────────────────────────────────────────────
impl_num!(
    int: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize;
    float: f32, f64
);
