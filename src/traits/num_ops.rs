// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
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
macro_rules! impl_num_ops {
    ($($t:ty),* $(,)?) => {$(
        impl NumOps for $t {}
    )*};
}

// ── `trait NumOps` Definition ───────────────────────────────────────────────
pub trait NumOps<Rhs = Self, Output = Self>:
    Sized
    + Add<Output = Self>
    + AddAssign<Rhs>
    + Sub<Output = Self>
    + SubAssign<Rhs>
    + Mul<Output = Self>
    + MulAssign<Rhs>
    + Div<Output = Self>
    + DivAssign<Rhs>
    + Rem<Output = Self>
    + RemAssign<Rhs>
    {}

// ── `NumOps` Implementations ────────────────────────────────────────────────
impl_num_ops!(
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
    usize,
    f32,
    f64
);
