// ── Aliases ─────────────────────────────────────────────────────────────────
extern crate std;

use core::cmp::Ordering;
use core::hash::BuildHasher;
use core::mem::{align_of, size_of};
use std::format;
use std::hash::RandomState;

use super::{
    HALF_USIZE,
    UsizeNoHighBit
};

// ── Constants ───────────────────────────────────────────────────────────────
const HIGH_BIT: usize = 1 << (usize::BITS - 1);
const MAX: usize = isize::MAX as usize;

// ── Macros ──────────────────────────────────────────────────────────────────
/// Asserts that `$wrapped` and `$raw` render identically under every listed
/// format spec. Specs must be string literals because `format!` requires them.
macro_rules! assert_debug_forwards {
    ($wrapped:expr, $raw:expr; $($spec:literal),+ $(,)?) => {$(
        assert_eq!(
            format!($spec, $wrapped),
            format!($spec, $raw),
            "mismatch for format spec {:?}",
            $spec,
        );
    )+};
}

// ── `HALF_USIZE` Tests ──────────────────────────────────────────────────────
#[test]
fn test_half_usize() {
    assert_eq!(HALF_USIZE, MAX);
    assert_eq!(HALF_USIZE & HIGH_BIT, 0);
    assert_eq!(HALF_USIZE | HIGH_BIT, usize::MAX);
}

// ── `UsizeNoHighBit` Tests ──────────────────────────────────────────────────
#[test]
fn test_usize_no_high_bit_new() {
    assert_eq!(UsizeNoHighBit::new(0).map(UsizeNoHighBit::as_inner), Some(0));
    assert_eq!(
        UsizeNoHighBit::new(isize::MAX as usize).map(UsizeNoHighBit::as_inner),
        Some(isize::MAX as usize)
    );
    assert_eq!(UsizeNoHighBit::new(isize::MAX as usize + 1), None);
    assert_eq!(UsizeNoHighBit::new(usize::MAX), None);
}

#[test]
fn test_usize_no_high_bit_new_accepts_every_low_bit() {
    for shift in 0..usize::BITS - 1 {
        let val = 1usize << shift;
        assert_eq!(
            UsizeNoHighBit::new(val).map(UsizeNoHighBit::as_inner),
            Some(val),
            "rejected 1 << {shift}"
        );
    }
}

#[test]
fn test_usize_no_high_bit_new_rejects_high_bit() {
    assert_eq!(UsizeNoHighBit::new(HIGH_BIT), None);
    assert_eq!(UsizeNoHighBit::new(usize::MAX - 1), None);

    for shift in 0..usize::BITS - 1 {
        assert_eq!(
            UsizeNoHighBit::new(HIGH_BIT | (1 << shift)),
            None,
            "accepted HIGH_BIT | 1 << {shift}"
        );
    }
}

#[test]
fn test_usize_no_high_bit_new_unchecked() {
    unsafe {
        assert_eq!(
            UsizeNoHighBit::new_unchecked(0).0,
            0
        );

        assert_eq!(
            UsizeNoHighBit::new_unchecked(isize::MAX as usize).0,
            isize::MAX as usize
        );
    };
}

#[test]
fn test_usize_no_high_bit_new_unchecked_round_trip() {
    for raw in [0, 1, HALF_USIZE >> 1, MAX] {
        // SAFETY: every value is in `0..=HALF_USIZE`.
        let val = unsafe { UsizeNoHighBit::new_unchecked(raw) };
        assert_eq!(val.as_inner(), raw);
        assert_eq!(Some(val), UsizeNoHighBit::new(raw));
    }
}

#[test]
fn test_usize_no_high_bit_as_inner() {
    assert_eq!(
        UsizeNoHighBit::new(0).unwrap().as_inner(),
        0
    );

    assert_eq!(
        UsizeNoHighBit::new(isize::MAX as usize).unwrap().as_inner(),
        isize::MAX as usize
    );
}

#[test]
fn test_usize_no_high_bit_const_eval() {
    const ZERO: Option<UsizeNoHighBit> = UsizeNoHighBit::new(0);
    const TOO_BIG: Option<UsizeNoHighBit> = UsizeNoHighBit::new(HIGH_BIT);
    const INNER: usize = match UsizeNoHighBit::new(MAX) {
        Some(val) => val.as_inner(),
        None => panic!("isize::MAX is in range"),
    };
    // SAFETY: 7 is in `0..=HALF_USIZE`.
    const UNCHECKED: UsizeNoHighBit = unsafe { UsizeNoHighBit::new_unchecked(7) };

    assert_eq!(ZERO.map(UsizeNoHighBit::as_inner), Some(0));
    assert_eq!(TOO_BIG, None);
    assert_eq!(INNER, MAX);
    assert_eq!(UNCHECKED.as_inner(), 7);
}

#[test]
fn test_usize_no_high_bit_layout() {
    assert_eq!(size_of::<UsizeNoHighBit>(), size_of::<usize>());
    assert_eq!(align_of::<UsizeNoHighBit>(), align_of::<usize>());
}

// ── Derived Trait Tests ─────────────────────────────────────────────────────
#[test]
fn test_usize_no_high_bit_clone_copy() {
    let a = UsizeNoHighBit::new(42).unwrap();
    let b = a; // `a` stays usable below only because the type is `Copy`.
    #[allow(clippy::clone_on_copy)]
    let c = a.clone();

    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn test_usize_no_high_bit_eq() {
    let a = UsizeNoHighBit::new(42).unwrap();
    assert_eq!(a, UsizeNoHighBit::new(42).unwrap());
    assert_ne!(a, UsizeNoHighBit::new(43).unwrap());
}

#[test]
fn test_usize_no_high_bit_ord() {
    let raws = [0, 1, 42, 1 << (usize::BITS - 2), MAX];

    // Ordering must agree with the inner `usize` for every pair.
    for &x in &raws {
        for &y in &raws {
            let a = UsizeNoHighBit::new(x).unwrap();
            let b = UsizeNoHighBit::new(y).unwrap();
            assert_eq!(a.cmp(&b), x.cmp(&y), "cmp({x}, {y})");
            assert_eq!(a.partial_cmp(&b), Some(x.cmp(&y)), "partial_cmp({x}, {y})");
        }
    }

    let zero = UsizeNoHighBit::new(0).unwrap();
    let one = UsizeNoHighBit::new(1).unwrap();
    let max = UsizeNoHighBit::new(MAX).unwrap();

    assert_eq!(zero.cmp(&max), Ordering::Less);
    assert_eq!(zero.max(max), max);
    assert_eq!(one.clamp(zero, one), one);

    let mut arr = [max, zero, one];
    arr.sort_unstable();
    assert_eq!(arr, [zero, one, max]);
}

#[test]
fn test_usize_no_high_bit_hash() {
    let state = RandomState::new();
    let a = UsizeNoHighBit::new(42).unwrap();
    let b = UsizeNoHighBit::new(42).unwrap();
    assert_eq!(state.hash_one(a), state.hash_one(b));
}

// ── `Debug` Tests ───────────────────────────────────────────────────────────
#[test]
fn test_usize_no_high_bit_debug_trait_fmt() {
    // Prints the bare value, not `UsizeNoHighBit(..)`.
    assert_eq!(format!("{:?}", UsizeNoHighBit::new(0).unwrap()), "0");
    assert_eq!(format!("{:?}", UsizeNoHighBit::new(42).unwrap()), "42");
    assert_eq!(format!("{:x?}", UsizeNoHighBit::new(255).unwrap()), "ff");
    assert_eq!(format!("{:?}", UsizeNoHighBit::new(MAX).unwrap()), format!("{MAX}"));

    // Formatter flags are forwarded to `usize`'s `Debug` impl.
    for raw in [0, 1, 42, 255, 1 << (usize::BITS - 2), MAX] {
        let wrapped = UsizeNoHighBit::new(raw).unwrap();
        assert_debug_forwards!(wrapped, raw;
            "{:?}", "{:#?}",
            "{:5?}", "{:<5?}", "{:^5?}", "{:>5?}", "{:*^9?}",
            "{:05?}", "{:+?}", "{:+06?}",
            "{:x?}", "{:X?}", "{:#x?}", "{:#X?}", "{:#010x?}",
        );
    }
}

#[test]
fn test_usize_no_high_bit_debug_trait_fmt_nested() {
    assert_eq!(format!("{:?}", UsizeNoHighBit::new(3)), "Some(3)");
    assert_eq!(format!("{:?}", UsizeNoHighBit::new(usize::MAX)), "None");

    let arr = [UsizeNoHighBit::new(1).unwrap(), UsizeNoHighBit::new(2).unwrap()];
    let raw = [1usize, 2];

    assert_eq!(format!("{arr:?}"), "[1, 2]");
    assert_eq!(format!("{arr:#?}"), format!("{raw:#?}"));
    // Containers pass their formatter options down to each element.
    assert_eq!(format!("{arr:>3?}"), format!("{raw:>3?}"));
}
