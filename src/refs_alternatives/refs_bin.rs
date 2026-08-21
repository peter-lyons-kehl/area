use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// @TODO could be Clone

/// Intentionally _not_ [Clone].
pub struct RefBin<'a, T, _AWI: AddrWidthIndicator> {
    r: &'a T,
    _awi: PhantomData<_AWI>,
}

// Re-export, primarily for alternative use switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

impl<'a, T, _AWI: AddrWidthIndicator> Deref for RefBin<'a, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.r
    }
}
