use crate::Area;
use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// @TODO could be Clone

/// Intentionally _not_ [Clone].
pub struct RefBin<'ta, T, _AWI: AddrWidthIndicator> {
    a: &'ta Area<_AWI>, //@TODO <- unsure

    ref_t: &'ta T,
    // _awi: PhantomData<_AWI>,
}

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

// @TODO consider: Instead of T, define this only for a Leaf<T>.
impl<'_ta, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_ta, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait Resolvable22 {
    type TO;
    fn resolve(&self) -> &Self::TO;
}
