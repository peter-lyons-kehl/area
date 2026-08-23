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
pub struct RefBin<'a, 't: 'a, T, ARI: AddrReprIndicator> {
    ref_t: &'t T,

    pub(crate) area: <ARI as AddrReprIndicator>::AreaRef<'a>,
}

impl<'_a, '_t: '_a, T, _AWI: AddrReprIndicator> Deref for RefBin<'_a, '_t, T, _AWI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

// @TODO rename? --> LoadImmediate, LoadParts, LoadWithParts, LoadNear, LoadTangernt, LoadJoined, LoadVerges, LoadTouching, LoadNigh
pub trait LoadDirect {
    type To<'a, 't: 'a, T: 't, ARI: AddrReprIndicator>
    where
        Self: 't,
        ARI: 'a;

    /// -> *value*/passable object, with referenced based on [RefBin]
    fn from<'a, 't: 'a, T, ARI: AddrReprIndicator>(
        &self,
        area: <ARI as AddrReprIndicator>::AreaRef<'a>,
    ) -> Self::To<'a, 't, T, ARI>;
    // \---> @TODO Docs:
    //
    // -> LinkedListNodeBinBased, returned as an instance
    //
    // @TODO should it have receiver with a lifetime?: &'t self
}

/// @TODO @TODO USELESS. What can this actually resolve?
impl<'a, 't: 'a, T, ARI: AddrReprIndicator> RefBin<'a, 't, T, ARI>
where
    T: LoadDirect,
{
    /// A shorter alternative to [LoadDirect::from] for [RefBin].
    ///
    /// This is why [AddrReprIndicator::AreaRef] has to be [Copy].
    pub fn from(&'t self) -> <T as LoadDirect>::To<'a, 't, T, ARI> {
        LoadDirect::from(/**/ self.ref_t /**/, self.area)
    }
}
