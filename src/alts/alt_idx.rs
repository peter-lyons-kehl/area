use crate::Area;
use crate::address::AddrReprIndicator;
use crate::alts::alt_bin::RefBin;
use core::marker::PhantomData;
use core::ops::Deref;

// @TODO Once https://github.com/rust-lang/rust/issues/135806 is stabilized, use
// core::marker::PhantomCovariantLifetime and friends.
/// VoR = ValueOrRef
#[repr(transparent)]
#[non_exhaustive]
pub struct VoR<'i, I>(I, PhantomData<&'i ()>);

impl<'i, I> Deref for VoR<'i, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// @TODO consider [Clone], but only for non-static lifetime 'a - so that it's tied to an [Area] by a
// lifetime.
//
// - if no other way, have two conflicting impl of Clone: one blanket for 'static, or for Any?

/// Intentionally _not_ [Clone]. @TODO Enable Clone for 'static, or even for any - since now it's invariant.
///
/// This type is invariant over lifetime '_a, so that `'static` couldn't be accidentally or
/// intentionally used in place of the expected lifetime. Invariant is ensured by [PhantomData] over
/// `fn(&'a ())`. See https://doc.rust-lang.org/nomicon/subtyping.html.
#[repr(C)]
pub struct RefIdx<'_a, '_t: '_a, _T, ARI: AddrReprIndicator> {
    /// This "becomes" [crate::alts::alt_bin::RefBin::ref_t] when `ARI` is
    /// [crate::address::AddrPtrWidthS].
    address: <ARI as AddrReprIndicator>::Addr,

    _a: PhantomData<&'_a ()>,
    _t_lifetime: PhantomData<&'_t ()>,
    _t_type: PhantomData<_T>,
    _invariant: PhantomData<fn(&'_a ())>,
}

// Re-export, primarily for alternative use switching between [RefIdx] and [refs_bin::RefBin] in
// client's code
pub use RefIdx as Ref;

/// This trait exists on its own, rather than just implementing [LoadFromArea::load_from] directly
/// for [RefIdx], so that we can also have the other function with same name [RefIdx::load_from]
/// implemented directly for [RefIdx] (with `'static` generic lifetimes), so that those two methods
/// then intentionally conflict if attempted to be used on [RefIdx] with `'static` lifetime.
pub trait LoadFromArea<'a, 't: 'a, T, ARI: AddrReprIndicator> {
    fn load_from(&self, a: <ARI as AddrReprIndicator>::AreaRef<'a>) -> RefBin<'a, 't, T, ARI>;
    // \---> @TODO seal the trait
    //
    // \---> @TODO should the receiver have 't lifetime??: &'t self
    //
    // verification + pointer arithmetic + cast + wrap
}

impl<'a, 't: 'a, T, ARI: AddrReprIndicator> LoadFromArea<'a, 't, T, ARI>
    for RefIdx<'a, 't, T, ARI>
{
    fn load_from(&self, a: <ARI as AddrReprIndicator>::AreaRef<'a>) -> RefBin<'a, 't, T, ARI> {
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

/// For more manual/fine grain resolving.
pub trait LoadByNeighbor<'a, 't: 'a, T: 't, ARI: AddrReprIndicator> {
    type To
    where
        Self: 't;

    /// Like [super::alt_bin::LoadDirect::from], but when we don't need to resolve other fields of
    /// the neighbor object, and (with a little inconvenience of passing in a `neighbor`) we resolve
    /// just a specific field (that is present as [RefIdx]).
    fn load_by(&'t self, neighbor: RefBin<'a, 't, T, ARI>) -> Self::To;
}
// @TODO \---- for what type to implement?

impl<'a, 't: 'a, T: 't, ARI: AddrReprIndicator> LoadByNeighbor<'a, 't, T, ARI>
    for RefIdx<'a, 't, T, ARI>
where
    Self: 't,
{
    type To = RefBin<'a, 't, T, ARI>;

    fn load_by(&'t self, neighbor: RefBin<'a, 't, T, ARI>) -> Self::To {
        self.load_from(neighbor.area)
    }
}

/* @TODO - if ever
impl<'a, T, ARI: AddrReprIndicator> RefIdx<'a, 'static, T, ARI> {
    // Intentionally conflicting with [LoadFromArea::load_from] if `'t` is `'static`, so that it
}*/
