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

    area: <AWI as AddrWidthIndicator>::AreaRef<'a>,
}

impl<'_a, '_t: '_a, T, _AWI: AddrWidthIndicator> Deref for RefBin<'_a, '_t, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    type To<'a, 't: 'a, T: 't, AWI: AddrWidthIndicator>
    where
        Self: 't,
        AWI: 'a;

    /// -> *value*/passable object, with referenced based on [RefBin]
    fn from<'a, 't: 'a, T, AWI: AddrWidthIndicator>(
        &self,
        area: <AWI as AddrWidthIndicator>::AreaRef<'a>,
    ) -> Self::To<'a, 't, T, AWI>;
    // \---> @TODO Docs:
    //
    // -> LinkedListNodeBinBased, by *value*
    //
    // @TODO should it have receiver with a lifetime?: &'t self
}

impl<'a, 't: 'a, T, AWI: AddrWidthIndicator> RefBin<'a, 't, T, AWI>
where
    T: ResolvableKids,
{
    /// A shorter alternative to [ResolvableKids::from] for [RefBin].
    ///
    /// This is why [AddrWidthIndicator::AreaRef] has to be [Copy].
    pub fn from(&'t self) -> <T as ResolvableKids>::To<'a, 't, T, AWI> {
        ResolvableKids::from(/**/ self.ref_t /**/, self.area)
    }
}
