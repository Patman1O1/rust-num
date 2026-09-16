// ── Core Aliases ────────────────────────────────────────────────────────────
use core::{
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
        Mul,
        MulAssign,
        Div,
        DivAssign,
        Neg
    }
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `Unum` Definition ───────────────────────────────────────────────────────
pub trait Unum:
    Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
}

// ── `Unum<T>` Implementation ────────────────────────────────────────────────
impl<T> Unum for T
where
    T: Sized
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
{
}

// ── `Inum` Definition ───────────────────────────────────────────────────────
pub trait Inum: Unum + Neg {}

// ── `Inum<T>` Implementation ────────────────────────────────────────────────
impl<T> Inum for T
where
    T: Unum + Neg
{
}
