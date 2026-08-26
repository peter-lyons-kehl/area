use crate::Area;
use crate::address::AddrReprIndicator;
use crate::alts::alt_bin::RefBin;
use core::convert::AsRef;
use core::marker::PhantomData;
use core::ops::Deref;

// Re-export, primarily for alternative use switching between [RefIdx] and [refs_bin::RefBin] in
// client's code
pub use self::{RefIdx as Ref, VorIdx as Vor};

// @TODO Once https://github.com/rust-lang/rust/issues/135806 is stabilized, use
// core::marker::PhantomCovariantLifetime and friends.
//
// @TODO #[repr(packed)] ??
/// VoR = ValueOrRef
#[repr(transparent)]
#[non_exhaustive]
pub struct VorIdx<'i, I>(I, PhantomData<&'i ()>);

impl<'i, I> Deref for VorIdx<'i, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'i, I> AsRef<I> for VorIdx<'i, I> {
    fn as_ref(&self) -> &I {
        &self.0
    }
}

/// It's OK to be [Clone], and even [Copy], since [RefIdx] is invariant over 'a.
///
/// This type is invariant over lifetime '_a, so that `'static` couldn't be accidentally or
/// intentionally used in place of the expected lifetime. Invariant is ensured by [PhantomData] over
/// `&'_a fn(&'_a ())`. See https://doc.rust-lang.org/nomicon/subtyping.html.
#[repr(C)]
pub struct RefIdx<'_a, _T, ARI: AddrReprIndicator<'_a>> {
    /// This "becomes" [crate::alts::alt_bin::RefBin::ref_t] when `ARI` is
    /// [crate::address::AddrPtrWidthS].
    pub(crate) address: <ARI as AddrReprIndicator<'_a>>::Addr,

    _a_invariant: PhantomData<&'_a fn(&'_a ())>,
    _t_type: PhantomData<_T>,
}
// No derive, since we want [Clone] regardless of whether _T is [Clone].
impl<'_a, _T, ARI: AddrReprIndicator<'_a>> Clone for RefIdx<'_a, _T, ARI> {
    fn clone(&self) -> Self {
        *self
        /*Self {
            address: self.address,
            _a_invariant: PhantomData,
            _t_type: PhantomData,
        }*/
    }
}
// No derive, since we want [Copy] regardless of whether _T is [Copy].
impl<'_a, _T, ARI: AddrReprIndicator<'_a>> Copy for RefIdx<'_a, _T, ARI> {}

// @TODO move out of trait, direct into RefIdx
/// This trait exists on its own, rather than just implementing [LoadFromArea::load_from] directly
/// for [RefIdx], so that we can also have the other function with same name [RefIdx::load_from]
/// implemented directly for [RefIdx] (with `'static` generic lifetimes), so that those two methods
/// then intentionally conflict if attempted to be used on [RefIdx] with `'static` lifetime.
pub trait LoadFromArea<'a, T, ARI: AddrReprIndicator<'a>> {
    fn load_from(&self, a: <ARI as AddrReprIndicator<'a>>::AreaRef) -> RefBin<'a, T, ARI>;
    // \---> @TODO seal the trait
    //
    // \---> @TODO should the receiver have 't lifetime??: &'t self
    //
    // verification + pointer arithmetic + cast + wrap
}

impl<'a, T, ARI: AddrReprIndicator<'a>> LoadFromArea<'a, T, ARI> for RefIdx<'a, T, ARI> {
    fn load_from(&self, a: <ARI as AddrReprIndicator<'a>>::AreaRef) -> RefBin<'a, T, ARI> {
        todo!()
    }
}

// @TODO this fails with "invariant"
//
/*impl<'a, T, ARI: AddrReprIndicator> RefIdx<'a, 'static, T, ARI> {
    /// Intentionally conflicting with [LoadFromArea::load_from] if `'t` is `'static`, so that it
    /// DOES conflict when the user tries to (possibly incorrectly) use a `'static`-based [RefIdx]
    /// with an [Area] where that [RefIdx] does _not_ resolve. See also [LoadFromArea::load_from].
    pub fn load_from<'t: 'a>(&self, _: <ARI as AddrReprIndicator>::AreaRef<'a>) -> RefBin<'a, 't, T, ARI> {
        unreachable!("RefIdx::load_from() is unsupported for 'static Area")
    }
}*/

// @TODO move out of the trait, into RefIdx
/// For more manual/fine grain resolving.
pub trait LoadByNeighbor<'a, 't: 'a, T: 't, ARI: AddrReprIndicator<'a>> {
    type To;

    /// Like [super::alt_bin::Loadable::load], but when we don't need to resolve other fields of
    /// the neighbor object, and (with a little inconvenience of passing in a `neighbor`) we resolve
    /// just a specific field (that is present as [RefIdx]).
    fn load_by(&'t self, neighbor: RefBin<'a, T, ARI>) -> Self::To;
}
// @TODO \---- for what type to implement?

impl<'a, 't: 'a, T: 't, ARI: AddrReprIndicator<'a>> LoadByNeighbor<'a, 't, T, ARI>
    for RefIdx<'a, T, ARI>
/*where
Self: 't,*/
{
    type To = RefBin<'a, T, ARI>;

    fn load_by(&'t self, neighbor: RefBin<'a, T, ARI>) -> Self::To {
        self.load_from(neighbor.area)
    }
}

/* @TODO - if ever
impl<'a, T, ARI: AddrReprIndicator> RefIdx<'a, 'static, T, ARI> {
    // Intentionally conflicting with [LoadFromArea::load_from] if `'t` is `'static`, so that it
}*/

pub trait EnsureInvariant<'a> {
    //fn self_outlives_a<'s: 'a>(&'s self) ->;

    //fn a_outlives_self(a: &'a Self) -> &impl EnsureInvariant<'a>;

    //type Selfie: EnsureInvariant<'a>;

    //type F: Fn(&'a ());

    //type Fm: FnMut()
}
impl<'a, 'i, I> EnsureInvariant<'a> for VorIdx<'i, I>
/*where
Self: 'a,*/
{
    //fn self_outlives_a<'s: 'a>(&'s self) {}

    /*fn a_outlives_self(a: &'a Self) -> &Self {
        a
    }*/

    //type Selfie = VoRIdx<'i, I>;
    //
    //type Selfie = Self;

    //type F =
}

pub trait ExtendLifetime {}

/// 1. Make sealed
/// 2. blanket impl only for fn(...)
pub trait ExtendFn<'a>: Fn(&'a ()) {
    type S: ExtendFn<'a>;
}
