use crate::Area;
use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

/// Intentionally _not_ [Clone].
pub struct RefBin<'ta, T, _AWI: AddrWidthIndicator> {
    area: &'ta Area<_AWI>, //@TODO <- unsure

    ref_t: &'ta T,
    // _awi: PhantomData<_AWI>,
}

// @TODO consider: Instead of T, define this only for a Leaf<T>.
impl<'_ta, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_ta, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    type TO;
    fn resolve_kids<AWI: AddrWidthIndicator>(&self, area: &Area<AWI>) -> &Self::TO;
}

impl<'_ta, T, _AWI: AddrWidthIndicator> RefBin<'_ta, T, _AWI>
where
    T: ResolvableKids,
{
    /// An alternative to [ResolvableKids::resolve_kids], in case `T` type itself, or its
    /// another trait, also has a `resolve` method (which would then conflict with
    /// [ResolvableKids::resolve_kids] if trait [ResolvableKids] were imported).
    pub fn resolve_kids<'ta, AWI: AddrWidthIndicator>(
        this: &'ta Self,
        area: &'ta Area<AWI>,
    ) -> &'ta <T as ResolvableKids>::TO {
        ResolvableKids::resolve_kids(this.ref_t, area)
    }
}
