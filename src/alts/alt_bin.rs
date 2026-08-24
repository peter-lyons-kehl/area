use crate::Area;
use crate::address::AddrReprIndicator;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use self::{RefBin as Ref, VoRBin as VoR};

/// VoR = ValueOrRef
#[repr(transparent)]
#[non_exhaustive]
pub struct VoRBin<'i, I>(&'i I);
impl<'i, I> VoRBin<'i, I> {
    pub fn new(ri: &'i I) -> Self {
        Self(ri)
    }
}

impl<'i, I> Deref for VoR<'i, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// Intentionally _not_ [Clone].
#[repr(C)]
pub struct RefBin<'a, 't: 'a, T, ARI: AddrReprIndicator> {
    ref_t: &'t T,

    pub(crate) area: <ARI as AddrReprIndicator>::AreaRef<'a>,
}

impl<'_a, '_t: '_a, T, _ARI: AddrReprIndicator> Deref for RefBin<'_a, '_t, T, _ARI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

// @TODO rename? --> LoadImmediate, LoadParts, LoadWithParts, LoadNear, LoadTangernt, LoadJoined, LoadVerges, LoadTouching, LoadNigh
//
// --> Loadable
pub trait Loadable<'a, ARI: AddrReprIndicator> {
    /*type To<'a, 't: 'a, T: 't, ARI: AddrReprIndicator>
    where
        Self: 't,
        ARI: 'a;*/
    type To;

    /// -> *value*/passable object, with referenced based on [RefBin]
    /*fn load<'a, 't: 'a, T, ARI: AddrReprIndicator>(
        &self,
        area: <ARI as AddrReprIndicator>::AreaRef<'a>,
    ) -> Self::To<'a, 't, T, ARI>;
    */
    fn load(&self, area: <ARI as AddrReprIndicator>::AreaRef<'a>) -> Self::To;

    // \---> @TODO Docs:
    //
    // -> LinkedListNodeBinBased, returned as an instance
    //
    // @TODO should it have receiver with a lifetime?: &'t self
}

/// @TODO @TODO USELESS. What can this actually resolve?
impl<'a, 't: 'a, T, ARI: AddrReprIndicator> RefBin<'a, 't, T, ARI>
where
    T: Loadable<'a, ARI>,
{
    /// A shorter alternative to [Loadable::load] for [RefBin].
    ///
    /// This is why [AddrReprIndicator::AreaRef] has to be [Copy].
    pub fn load(&'t self) -> <T as Loadable<'a, ARI>>::To {
        Loadable::load(/**/ self.ref_t /**/, self.area)
    }
}
