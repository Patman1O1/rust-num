// ── Aliases ─────────────────────────────────────────────────────────────────
use super::{
    num::Num
};

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! impl_real {
    ($($T:ty),* $(,)?) => {$(
        impl Real for $T {
            const MIN: Self = <$T>::MIN;

            const MAX: Self = <$T>::MAX;
        }
    )*};
}

// ── `trait Real` Definition ─────────────────────────────────────────────────
pub trait Real: Num {
    // ── Constants ───────────────────────────────────────────────────────────
    const MIN: Self;

    const MAX: Self;
}

// ── `Real` Implementations ──────────────────────────────────────────────────
impl_real!(
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
