use crate::Area;
use crate::address::{AddrReprIndicator, IntoUsize};
use crate::alts::alt_idx::{RefIdx, VorIdx};
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternatives by switching between [RefBin] and
// [crate::refs_alternatives::refs_idx::RefIdx] in client's code
pub use self::{RefBin as Ref, VorBin as Vor};

// @TODO Clone, or even Copy?
//
// @TODO #[repr(packed)] ??
/// VoR = ValueOrRef
#[repr(transparent)]
#[non_exhaustive]
pub struct VorBin<'a: 'i, 'i, I>(&'i I, PhantomData<&'a fn(&'a ()) /* invariant over 'a */>);

impl<'a: 'i, 'i, I> VorBin<'a, 'i, I> {
    // @TODO not necessry: for now: @TODO receive area: Option<&Area<'a>, ARI> instead of just a
    // generic ARI. This way, if it is Some, we can (optionally/in debug mode) verify that the
    // reference is within the given Area.
    pub fn new(ri: &'i I) -> Self {
        Self(ri, PhantomData)
    }
}

impl<'a: 'i, 'i, I> Deref for VorBin<'a, 'i, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
// @TODO \\--> AsRef, too? Like it is already so for VorIdx.

impl<'a: 'i, 'i, I> VorBin<'a, 'i, I> {
    // Previous, but unnecessary:
    //
    //, ARI: AddrReprIndicator + 'a>(
    //
    // vor_idx: &'i VorIdx<'a, I>,
    pub fn from_vor_idx(
        vor_idx: &VorIdx<'a, 'i, I>,
        //_area: <ARI as crate::address::AddrReprIndicator>::AreaRef<'a>,
    ) -> VorBin<'a, 'i, I> {
        // let result: : VoRBin<'i, I> = ...
        let result = VorBin::new(vor_idx.as_ref());

        unsafe { core::mem::transmute(result) }
    }

    pub fn extend_lifetime<ARI: AddrReprIndicator<'a>>(&'i self) -> &'a I {
        unsafe { core::mem::transmute(&self.0) }
    }
}

/// Intentionally _not_ [Clone]. @TODO consider [Clone], or even [Copy] - since now it's invariant over 'a.
#[repr(C)]
pub struct RefBin<'a, T, ARI: AddrReprIndicator<'a>> {
    ref_t: &'a T,

    //@TODO: pub(crate) + make a shortcut fn for what gets affected
    pub(crate) area: <ARI as AddrReprIndicator<'a>>::AreaRef,
    _a_invariant: PhantomData<&'a fn(&'a ())>,
}
impl<'a, T, ARI: AddrReprIndicator<'a>>
    From<(RefIdx<'a, T, ARI>, <ARI as AddrReprIndicator<'a>>::AreaRef)> for RefBin<'a, T, ARI>
{
    fn from(
        (ref_idx, area_ref): (RefIdx<'a, T, ARI>, <ARI as AddrReprIndicator<'a>>::AreaRef),
    ) -> Self {
        Self {
            ref_t: unsafe {
                core::mem::transmute(&area_ref.into().data[ref_idx.address.into_usize()])
            },
            area: area_ref,
            _a_invariant: PhantomData,
        }
    }
}

impl<'_a, '_t: '_a, T, _ARI: AddrReprIndicator<'_a>> Deref for RefBin<'_a, T, _ARI> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ref_t
    }
}

// @TODO rename? --> LoadImmediate, LoadParts, LoadWithParts, LoadNear, LoadTangernt, LoadJoined, LoadVerges, LoadTouching, LoadNigh
//
// --> Loadable
pub trait Loadable<'a: 'i, 'i, ARI: AddrReprIndicator<'a>> {
    /*type To<'a, 't: 'a, T: 't, ARI: AddrReprIndicator>
    where
        Self: 't,
        ARI: 'a;*/
    type To: 'a + 'i;

    /// -> *value*/passable object, with referenced based on [RefBin]
    /*fn load<'a, 't: 'a, T, ARI: AddrReprIndicator>(
        &self,
        area: <ARI as AddrReprIndicator>::AreaRef<'a>,
    ) -> Self::To<'a, 't, T, ARI>;
    */

    // @TODO:
    //
    // Without the leading lifetime 'a for the receiver (&'a self) we had difficulties to implement
    // it - see examples/01/types/def_etc.rs
    //
    // fn load(&'a self,...
    fn load_from(&self, area: <ARI as AddrReprIndicator<'a>>::AreaRef) -> Self::To;

    // \---> @TODO Docs:
    //
    // -> LinkedListNodeBinBased, returned as an instance
    //
    // @TODO should it have receiver with a lifetime?: &'t self
}

impl<'a: 'i, 'i, T, ARI: AddrReprIndicator<'a>> RefBin<'a, T, ARI>
where
    T: Loadable<'a, 'i, ARI>,
{
    /// A shorter alternative to [Loadable::load] for [RefBin].
    ///
    /// This is why [AddrReprIndicator::AreaRef] has to be [Copy].
    ///
    /// pub fn load(&'a self) -> ...
    pub fn load(&self) -> <T as Loadable<'a, 'i, ARI>>::To {
        Loadable::load_from(/**/ self.ref_t /**/, self.area)
    }
}
