use crate::Area;
use crate::address::AddrWidthIndicator;
use crate::refs_alternatives::refs_bin::RefBin;
use core::marker::PhantomData;

// @TODO consider [Clone], but only for non-static lifetime 'a - so that it's tied to an [Area] by a
// lifetime.
//
// - if no other way, have two conflicting impl of Clone: one blanket for 'static, or for Any?

/// Intentionally _not_ [Clone]. @TODO Enable Clone for 'static, or even for any - since now it's invariant.
///
/// This type is invariant over lifetime '_a, so that `'static` couldn't be accidentally or
/// intentionally used in place of the expected lifetime. That is ensured by [PhantomData] over
/// `fn(&'a ())`. See https://doc.rust-lang.org/nomicon/subtyping.html.
pub struct RefIdx<'_a, _T, AWI: AddrWidthIndicator> {
    address: <AWI as AddrWidthIndicator>::Addr,
    _a: PhantomData<&'_a ()>,
    _r: PhantomData<_T>,
    _invariant: PhantomData<fn(&'_a ())>,
}

// Re-export, primarily for alternative use switching between [RefIdx] and [refs_bin::RefBin] in
// client's code
pub use RefIdx as Ref;

trait Resolvable22 {
    fn of<'sa, T, AWI: AddrWidthIndicator>(
        &'sa self,
        a: &'sa Area<AWI>,
    ) -> RefBin<'sa, T, AWI>;
    // verification + pointer arithmetic + cast + wrap
}

impl<T, AWI: AddrWidthIndicator> RefIdx<'static, T, AWI> {
    // @TODO consider method: leaf_of(&self, a: &'a Area<AWI> -> RefBin<'a, Leaf<T>, AWI>, or even
    // direct -> &'a T. See crate::refs_alternatives::refs_bins.

    // @TODO KEEP this as a DUPLICATE FUNCTION to an any-lifetime-based function with the same name
    // IN A TRAIT, so that it DOES conflict when the user tries to (possibly incorrectly) use a
    // 'static-base RefIdx with an Area where that RefIdx doesn't resolve.
    pub fn of<'a>(&self, a: &'a Area<AWI>) -> RefBin<'a, T, AWI> {
        unreachable!("RefIdx::of() is unsupported for 'static")
    }
}

pub trait ResolvableChild {
    type To;

    /// Like [ResolvableKids::resolve], but when we don't need to resolve other fields of the parent
    /// object, and (with a little inconvenience of passing in the parent) we resolve just a
    /// specific field.
    //fn resolve_where<'_a, '_t: '_a, T, AWI: AddrWidthIndicator>(&'_t self, area: &Area<'_a, AWI>) -> Self::To;
    fn resolve_where<'_a, '_t: '_a, T, AWI: AddrWidthIndicator>(&'_t self, parent: RefBin<'_a, '_t, T, AWI>) -> Self::To;
}

//----
impl<'_a, T, AWI: AddrWidthIndicator> RefIdx<'_a, T, AWI>
where
    T: ResolvableChild,
{
    /// An alternative to [ResolvableKids::resolve], in case `T` type itself, or its
    /// another trait, also has a `resolve` method (which would then conflict with
    /// [ResolvableKids::resolve] if trait [ResolvableKids] were imported).
    pub fn resolve_where<'_t>(
        this: &'_t Self,
        area: &'_a Area<'_a, AWI>,
    ) -> <T as ResolvableChild>::To {
        let parent = loop {} as RefBin<'_a, '_t, T, AWI>;
        ResolvableChild::resolve_where(this, area)
    }
}