use crate::Area;
use crate::address::AddrReprIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use RefBin as Ref;

/// VoR = ValueOrRef
#[repr(transparent)]
#[non_exhaustive]
pub struct VoR<'i, I>(&'i I);

impl<'i, I> Deref for VoR<'i, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Intentionally _not_ [Clone].
#[repr(C)]
pub struct RefBin<'a, 't: 'a, T, AWI: AddrReprIndicator> {
    ref_t: &'t T,

    area: <AWI as AddrReprIndicator>::AreaRef<'a>,
}

impl<'_a, '_t: '_a, T, _AWI: AddrReprIndicator> Deref for RefBin<'_a, '_t, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

pub trait ResolvableKids {
    type To<'a, 't: 'a, T: 't, AWI: AddrReprIndicator>
    where
        Self: 't,
        AWI: 'a;

    /// -> *value*/passable object, with referenced based on [RefBin]
    fn from<'a, 't: 'a, T, AWI: AddrReprIndicator>(
        &self,
        area: <AWI as AddrReprIndicator>::AreaRef<'a>,
    ) -> Self::To<'a, 't, T, AWI>;
    // \---> @TODO Docs:
    //
    // -> LinkedListNodeBinBased, by *value*
    //
    // @TODO should it have receiver with a lifetime?: &'t self
}

impl<'a, 't: 'a, T, AWI: AddrReprIndicator> RefBin<'a, 't, T, AWI>
where
    T: ResolvableKids,
{
    /// A shorter alternative to [ResolvableKids::from] for [RefBin].
    ///
    /// This is why [AddrReprIndicator::AreaRef] has to be [Copy].
    pub fn from(&'t self) -> <T as ResolvableKids>::To<'a, 't, T, AWI> {
        ResolvableKids::from(/**/ self.ref_t /**/, self.area)
    }
}
