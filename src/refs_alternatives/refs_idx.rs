use crate::Area;
use crate::address::AddrWidthIndicator;
use crate::refs_alternatives::refs_bin;
use core::marker::PhantomData;

// @TODO consider [Clone], but only for non-static lifetime 'a - so that it's tied to an [Area] by a
// lifetime.
//
// - if no other way, have two conflicting impl of Clone: one blanket for 'static, or for Any?

/// Intentionally _not_ [Clone].
pub struct RefIdx<'_a, _T, AWI: AddrWidthIndicator> {
    bytes: <AWI as AddrWidthIndicator>::Addr,
    _a: PhantomData<&'_a ()>,
    _r: PhantomData<_T>,
}

// Re-export, primarily for alternative use switching between [RefIdx] and [refs_bin::RefBin] in
// client's code
pub use RefIdx as Ref;

impl<T, AWI: AddrWidthIndicator> RefIdx<'static, T, AWI> {
    pub fn of<'a>(a: &'a Area<AWI>) -> refs_bin::Ref<'a, T, AWI> {
        todo!()
    }
}
