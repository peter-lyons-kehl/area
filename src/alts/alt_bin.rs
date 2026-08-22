use crate::Area;
use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

/// Intentionally _not_ [Clone].
#[repr(C)]
pub struct RefBin<'a, 't: 'a, T, _AWI: AddrWidthIndicator> {
    area: &'a Area<'a, _AWI>, //@TODO <- unsure

    ref_t: &'t T,
    // _awi: PhantomData<_AWI>,
}

// @TODO consider: Instead of T, define this only for a Leaf<T>.
impl<'_a, '_t: '_a, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_a, '_t, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    //@TODO generic 'a, 't: 'a, T, _AWI: AddWidthIndicator
    type To;

    // @TODO Docs
    //
    /// -> *value*/passable object, with referenced based on [RefBin]
    ///
    // -> LinkedListNodeBinBased, by *value*
    fn from<AWI: AddrWidthIndicator>(&self, area: &Area<AWI>) -> &Self::To;
}

impl<'_a, '_t: '_a, T, _AWI: AddrWidthIndicator> RefBin<'_a, '_t, T, _AWI>
where
    T: ResolvableKids,
{
    /// An alternative to [ResolvableKids::from], in case `T` type itself, or its
    /// another trait, also has a `from` method (which would then conflict with
    /// [ResolvableKids::from] if trait [ResolvableKids] were imported).
    pub fn from<'ta, AWI: AddrWidthIndicator>(
        this: &'ta Self,
        area: &'ta Area<AWI>,
    ) -> &'ta <T as ResolvableKids>::To {
        ResolvableKids::from(this.ref_t, area)
    }
}
