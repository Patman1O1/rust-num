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

// ── `trait Num` Definition ──────────────────────────────────────────────────
pub trait Num:
    Sized
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
    // ── Types ───────────────────────────────────────────────────────────────
    type FromStrRadixErr;

    // ── Functions ───────────────────────────────────────────────────────────
    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr>;
}

// ── `Num` Implementations ───────────────────────────────────────────────────
