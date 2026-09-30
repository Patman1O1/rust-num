// ── Aliases ─────────────────────────────────────────────────────────────────
use super::{
    UsizeNoHighBit
};

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

// TODO
#[test]
fn test_usize_no_high_bit_as_inner() { todo!(); }

// TODO
#[test]
fn test_usize_no_high_bit_fmt() {
    todo!();
}
