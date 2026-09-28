// ── Aliases ─────────────────────────────────────────────────────────────────

// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── Macros ──────────────────────────────────────────────────────────────────
macro_rules! define_valid_range_type {
    ($(
        $(#[$m:meta])*
        $vis:vis struct $name:ident($int:ident is $pat:pat);
    )+) => {$(
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        $(#[$m])*
        $vis struct $name($int);

        impl $name {
            #[inline]
            pub const fn new(val: $int) -> ::core::option::Option<Self> {
                #[allow(non_contiguous_range_endpoints)]
                if let $pat = val {
                    ::core::option::Option::Some(Self(val))
                } else {
                    ::core::option::Option::None
                }
            }

            /// Constructs an instance without checking the range.
            ///
            /// # Safety
            /// `val` must match the type's pattern. Violating this is library UB:
            /// `as_inner` would then feed a false assumption to the optimizer.
            #[inline]
            pub const unsafe fn new_unchecked(val: $int) -> Self {
                Self(val)
            }

            #[inline]
            pub const fn as_inner(self) -> $int {
                // SAFETY: every constructor guarantees the value matches the pattern.
                #[allow(non_contiguous_range_endpoints)]
                unsafe {
                    ::core::hint::assert_unchecked(matches!(self.0, $pat));
                }
                self.0
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <$int as ::core::fmt::Debug>::fmt(&self.as_inner(), f)
            }
        }
    )+};
}

// ── Constants ───────────────────────────────────────────────────────────────
const HALF_USIZE: usize = usize::MAX >> 1;

// ── Types ───────────────────────────────────────────────────────────────────
define_valid_range_type! {
    pub struct UsizeNoHighBit(usize is 0..HALF_USIZE);
}

