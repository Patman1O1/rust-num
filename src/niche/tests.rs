// ── Aliases ─────────────────────────────────────────────────────────────────
use super::{
    UsizeNoHighBit
};

// ── UsizeNoHighBit Tests ────────────────────────────────────────────────────
#[test]
fn test_usize_no_high_bit_new() {
    assert_eq!(Option::Some, UsizeNoHighBit::new(0));
    
}
