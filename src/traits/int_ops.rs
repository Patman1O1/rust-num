// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    ops::{
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Not,
        Shl,
        ShlAssign,
        Shr,
        ShrAssign
    }
};

use super::{
    num_ops::NumOps
};
 

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_int_ops {
    ($($t:ty),* $(,)?) => {$(
        impl IntOps for $t {}
    )*};
}

// ── `trait IntOps` Definition ───────────────────────────────────────────────
pub trait IntOps<Rhs = Self, Output = Self>:
    Sized
    + NumOps
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Not
    + Shl
    + ShlAssign
    + Shr
    + ShrAssign {}

// ── `IntOps` Implementations ────────────────────────────────────────────────
impl_int_ops!(
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
