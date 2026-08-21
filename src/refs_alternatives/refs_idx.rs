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

trait Resolvable2 {
    fn of<'a, T, AWI: AddrWidthIndicator>(&self, a: &'a Area<AWI>) -> refs_bin::Ref<'a, T, AWI>;
}

impl<T, AWI: AddrWidthIndicator> RefIdx<'static, T, AWI> {
    // @TODO consider method: leaf_of(&self, a: &'a Area<AWI> -> RefBin<'a, Leaf<T>, AWI>, or even
    // direct -> &'a T. See crate::refs_alternatives::refs_bins.

    // @TODO KEEP this as a DUPLICATE FUNCTION to an any-lifetime-based function with the same name
    // IN A TRAIT, so that it DOES conflict when the user tries to (possibly incorrectly) use a
    // 'static-base RefIdx with an Area where that RefIdx doesn't resolve.
    pub fn of<'a>(&self, a: &'a Area<AWI>) -> refs_bin::Ref<'a, T, AWI> {
        todo!() // verification + pointer arithmetic + cast + wrap
    }
}
