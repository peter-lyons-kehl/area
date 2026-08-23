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

/// This trait exists on its own, rather than just implementing [Of::of] directly for [RefIdx], so
/// that we can also have the other function with same name [RefIdx::of] implemented directly for
/// [RefIdx] (with `'static` generic lifetimes), so that those two methods then intentionally
/// conflict if attempted to be used on [RefIdx] with `'static` lifetime.
pub trait Of<'a, 't: 'a, T, ARI: AddrReprIndicator> {
    fn of(&self, a: &'a Area<ARI>) -> RefBin<'a, 't, T, ARI>;
    // \---> @TODO seal the trait
    //
    // \---> @TODO should the receiver have 't lifetime??: &'t self
    //
    // verification + pointer arithmetic + cast + wrap
}

impl<'a, 't: 'a, T, ARI: AddrReprIndicator> Of<'a, 't, T, ARI> for RefIdx<'a, 't, T, ARI> {
    fn of(&self, a: &'a Area<ARI>) -> RefBin<'a, 't, T, ARI> {
        todo!()
    }
}

impl<'a, T, ARI: AddrReprIndicator> RefIdx<'a, 'static, T, ARI> {
    /// Intentionally conflicting with [Of::of] if `'t` is `'static`, so that it DOES conflict when
    /// the user tries to (possibly incorrectly) use a `'static`-based [RefIdx] with an [Area] where
    /// that [RefIdx] does _not_ resolve. See also [Of::of].
    pub fn of<'t: 'a>(&self, _: &'a Area<ARI>) -> RefBin<'a, 't, T, ARI> {
        unreachable!("RefIdx::of() is unsupported for 'static Area")
    }
}

/// For more manual/fine grain resolving.
pub trait ResolvableRelative {
    type To<'a, 't: 'a, T: 't, ARI: AddrReprIndicator>
    where
        Self: 't;

    /// Like [super::alt_bin::ResolvableKids::from], but when we don't need to resolve other fields
    /// of the relative object, and (with a little inconvenience of passing in a `relative`) we
    /// resolve just a specific field (that is present as [RefIdx]).
    fn by<'a, 't: 'a, T, ARI: AddrReprIndicator>(
        &'t self,
        relative: RefBin<'a, 't, T, ARI>,
    ) -> Self::To<'a, 't, T, ARI>;
}
// @TODO \---- for what type to implement?
