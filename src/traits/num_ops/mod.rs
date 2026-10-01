// ── Aliases ─────────────────────────────────────────────────────────────────
use core::{
    cmp::PartialEq,
    ops::{
        Add,
        Sub,
        Mul,
        Div,
        Rem
    }
};

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `trait NumOps` Definition ───────────────────────────────────────────────
pub trait NumOps<Rhs = Self, Output = Self>: 
    Add<Rhs, Output = Output> +
    Sub<Rhs, Output = Output> +
    Mul<Rhs, Output = Output> +
    Div<Rhs, Output = Output> +
    Rem<Rhs, Output = Output> +
    PartialEq{}
