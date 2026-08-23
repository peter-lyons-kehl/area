use crate::Area;
use crate::address::AddrWidthIndicator;
use crate::alts::alt_bin::RefBin;
use core::marker::PhantomData;

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
pub struct RefIdx<'_a, '_t: '_a, _T, AWI: AddrWidthIndicator> {
    /// This "becomes" [crate::alts::alt_bin::RefBin::ref_t] when `AWI` is
    /// [crate::address::AddrPtrWidthS].
    address: <AWI as AddrWidthIndicator>::Addr,

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
pub trait Of<'a, 't: 'a, T, AWI: AddrWidthIndicator> {
    fn of(&self, a: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI>;
    // \---> @TODO seal the trait
    //
    // \---> @TODO should the receiver have 't lifetime??: &'t self
    //
    // verification + pointer arithmetic + cast + wrap
}

impl<'a, 't: 'a, T, AWI: AddrWidthIndicator> Of<'a, 't, T, AWI> for RefIdx<'a, 't, T, AWI> {
    fn of(&self, a: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI> {
        todo!()
    }
}

impl<'a, T, AWI: AddrWidthIndicator> RefIdx<'a, 'static, T, AWI> {
    /// Intentionally conflicting with [Of::of] if `'t` is `'static`, so that it DOES conflict when
    /// the user tries to (possibly incorrectly) use a `'static`-based [RefIdx] with an [Area] where
    /// that [RefIdx] does _not_ resolve. See also [Of::of].
    pub fn of<'t: 'a>(&self, _: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI> {
        unreachable!("RefIdx::of() is unsupported for 'static Area")
    }
}

/// For more manual/fine grain resolving.
pub trait ResolvableRelative {
    type To<'a, 't: 'a, T: 't, AWI: AddrWidthIndicator>
    where
        Self: 't;

    /// Like [super::alt_bin::ResolvableKids::from], but when we don't need to resolve other fields
    /// of the relative object, and (with a little inconvenience of passing in a `relative``) we
    /// resolve just a specific field (that is present as [RefIdx]).
    fn by<'a, 't: 'a, T, AWI: AddrWidthIndicator>(
        &'t self,
        relative: RefBin<'a, 't, T, AWI>,
    ) -> Self::To<'a, 't, T, AWI>;
}
// @TODO \---- for what type to implement?
