use crate::Area;
use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

/// Intentionally _not_ [Clone].
pub struct RefBin<'a, 't: 'a, T, _AWI: AddrWidthIndicator> {
    area: &'a Area<'a, _AWI>, //@TODO <- unsure

    ref_t: &'t T,
    // _awi: PhantomData<_AWI>,
}

// @TODO consider: Instead of T, define this only for a Leaf<T>.
impl<'_a, '_t: '_a, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_a, 't, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    type To;

    // @TODO -> ValBin ??? = trait with a `type`?
    //
    // -> LinkedListNodeBinBased, by *value*
    fn resolve<AWI: AddrWidthIndicator>(&self, area: &Area<AWI>) -> &Self::To;
}

impl<'_ta, T, _AWI: AddrWidthIndicator> RefBin<'_ta, T, _AWI>
where
    T: ResolvableKids,
{
    /// An alternative to [ResolvableKids::resolve], in case `T` type itself, or its
    /// another trait, also has a `resolve` method (which would then conflict with
    /// [ResolvableKids::resolve] if trait [ResolvableKids] were imported).
    pub fn resolve<'ta, AWI: AddrWidthIndicator>(
        this: &'ta Self,
        area: &'ta Area<AWI>,
    ) -> &'ta <T as ResolvableKids>::To {
        ResolvableKids::resolve(this.ref_t, area)
    }
}
