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
macro_rules! impl_num {
    ($($t:ty),* $(,)?) => {$(
        impl Num for $t {
            const MIN: Self = <$t>::MIN;

            const MAX: Self = <$t>::MAX;
        }
    )*};
}

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
    // ── Constants ───────────────────────────────────────────────────────────
    const MIN: Self;

    const MAX: Self;
}

// ── `Num` Implementations ───────────────────────────────────────────────────
impl_num!(
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
