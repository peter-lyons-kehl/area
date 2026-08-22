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
pub struct RefIdx<'_a, '_t: '_a, _T, AWI: AddrWidthIndicator> {
    address: <AWI as AddrWidthIndicator>::Addr,
    _a: PhantomData<&'_a ()>,
    _t_lifetime: PhantomData<&'_t ()>,
    _t_type: PhantomData<_T>,
    _invariant: PhantomData<fn(&'_a ())>,
}

// Re-export, primarily for alternative use switching between [RefIdx] and [refs_bin::RefBin] in
// client's code
pub use RefIdx as Ref;

// @TODO consider remove the trait, have function `of` direct in RefIdx
trait Of<'a, 't: 'a, T, AWI: AddrWidthIndicator> {
    fn of(&self, a: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI>;
    // verification + pointer arithmetic + cast + wrap
}

impl<'a, 't: 'a, T, AWI: AddrWidthIndicator> Of<'a, 't, T, AWI> for RefIdx<'a, 't, T, AWI> {
    fn of(&self, a: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI> {
        todo!()
    }
}

impl<'a, T, AWI: AddrWidthIndicator> RefIdx<'a, 'static, T, AWI> {
    // @TODO consider method: leaf_of(&self, a: &'a Area<AWI> -> RefBin<'a, Leaf<T>, AWI>, or even
    // direct -> &'a T. See crate::refs_alternatives::refs_bins.

    /// Intentionally a DUPLICATE to an any-non-static-lifetime-based function [Of::of], so that it
    /// DOES conflict when the user tries to (possibly incorrectly) use a `'static`-based [RefIdx]
    /// with an [Area] where that [RefIdx] does _not_ resolve.
    pub fn of<'t: 'a>(&self, _: &'a Area<AWI>) -> RefBin<'a, 't, T, AWI> {
        unreachable!("RefIdx::of() is unsupported for 'static")
    }
}

// --> @TODO --> _not_ in a trait, but directly in RefIdx impl
pub trait ResolvableChild {
    type To;

    /// Like [super::refs_bin::ResolvableKids::resolve], but when we don't need to resolve other fields of the parent
    /// object, and (with a little inconvenience of passing in the parent) we resolve just a
    /// specific field (that is present as [RefIdx]).
    //fn by<'_a, '_t: '_a, T, AWI: AddrWidthIndicator>(&'_t self, area: &Area<'_a, AWI>) -> Self::To;
    fn by<'_a, '_t: '_a, T, AWI: AddrWidthIndicator>(
        &'_t self,
        parent: RefBin<'_a, '_t, T, AWI>,
    ) -> Self::To;
}
// @TODO \---- for what type to implement?
//
// --> @TODO --> _not_ in a trait, but directly in RefIdx impl

/// An alternative to [ResolvableChild::by], in case `T` type itself, or its another
/// trait, also has a `by` method (which would then conflict with
/// [ResolvableChild::by] if trait [ResolvableChild] were imported).
pub fn by<'_a, '_t: '_a, T: ResolvableChild, AWI: AddrWidthIndicator>(
    this: &'_t T,
    parent: RefBin<'_a, '_t, T, AWI>,
) -> <T as ResolvableChild>::To {
    let parent = todo!() as RefBin<'_a, '_t, T, AWI>;
    ResolvableChild::by(this, parent)
}
