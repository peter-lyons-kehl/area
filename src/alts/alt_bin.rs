use crate::Area;
use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

/// Intentionally _not_ [Clone].
#[repr(C)]
pub struct RefBin<'a, 't: 'a, T, AWI: AddrWidthIndicator> {
    ref_t: &'t T,

    area: &'a <AWI as AddrWidthIndicator>::AreaRef<'a>,
}

// @TODO consider: Instead of T, define this only for a Leaf<T>.
impl<'_a, '_t: '_a, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_a, '_t, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    type To<'a, 't: 'a, T: 't, AWI: AddrWidthIndicator>
    where
        Self: 't;

    // @TODO Docs
    //
    /// -> *value*/passable object, with referenced based on [RefBin]
    ///
    // -> LinkedListNodeBinBased, by *value*
    //
    // @TODO should it have receiver with a lifetime?: &'t self
    fn from<'a, 't: 'a, T, AWI: AddrWidthIndicator>(
        &self,
        area: &Area<AWI>,
    ) -> Self::To<'a, 't, T, AWI>;
}

impl<'a, 't: 'a, T, AWI: AddrWidthIndicator> RefBin<'a, 't, T, AWI>
where
    T: ResolvableKids,
{
    /// An alternative to [ResolvableKids::from], in case `T` type itself, or its
    /// another trait, also has a `from` method (which would then conflict with
    /// [ResolvableKids::from] if trait [ResolvableKids] were imported).
    pub fn from(this: &'t Self, area: &'a Area<AWI>) -> <T as ResolvableKids>::To<'a, 't, T, AWI> {
        ResolvableKids::from(this.ref_t, area)
    }
}
