/// Intentionally private.
struct Empty;

pub trait NoClone {
    #[allow(private_interfaces)]
    fn assure_no_clone(&self) -> Empty {
        Empty
    }

    #[allow(private_interfaces)]
    fn assure_no_clone_assoc() -> Empty
    where
        Self: Sized,
    {
        Empty
    }
}
fn _assure_no_clone_is_dyn_compatible(nc: &dyn NoClone) {
    nc.assure_no_clone();
}

unsafe extern "C" {
    fn area_non_clone_cant_be_assured();
}

impl<T: Clone> NoClone for T {
    #[allow(private_interfaces)]
    fn assure_no_clone(&self) -> Empty {
        unsafe {
            area_non_clone_cant_be_assured();
        }
        panic!("Unsupported")
    }

    #[allow(private_interfaces)]
    fn assure_no_clone_assoc() -> Empty
    where
        Self: Sized,
    {
        unsafe {
            area_non_clone_cant_be_assured();
        }
        panic!("Unsupported")
    }
}

#[cfg(test)]
mod tests {
    use crate::NoClone;

    #[test]
    fn without_clone() {
        struct S {}
        impl NoClone for S {}
        let s = S {};

        fn pass_no_clone(nc: &dyn NoClone) -> &dyn NoClone {
            core::hint::black_box(nc)
        }
        pass_no_clone(&s).assure_no_clone();
    }
}

pub trait Referrable {}

/** Pointer width choice. As if an enum. */
pub type PtrWidthChoice = u8;
pub const PTR_WIDTH_CHOICE_2: PtrWidthChoice = 0;
pub const PTR_WIDTH_CHOICE_4: PtrWidthChoice = 1;
pub const PTR_WIDTH_CHOICE_8: PtrWidthChoice = 2;
/// `usize`-like
pub const PTR_WIDTH_CHOICE_S: PtrWidthChoice = 3;

/** Pointer width, in bytes. */
pub type PtrWidth = usize;
pub const PTR_WIDTH_2: PtrWidth = 2;
pub const PTR_WIDTH_4: PtrWidth = 4;
pub const PTR_WIDTH_8: PtrWidth = 8;

pub type Align = usize;
pub type CacheLineWidth = usize;

trait PtrChoiceWidth {
    const PTR_WIDTH: PtrWidth;
}

struct PtrChoiceToWidth<const C: PtrWidthChoice> {}
impl PtrChoiceWidth for PtrChoiceToWidth<PTR_WIDTH_CHOICE_2> {
    const PTR_WIDTH: PtrWidth = PTR_WIDTH_2;
}
impl PtrChoiceWidth for PtrChoiceToWidth<PTR_WIDTH_CHOICE_4> {
    const PTR_WIDTH: PtrWidth = PTR_WIDTH_4;
}
impl PtrChoiceWidth for PtrChoiceToWidth<PTR_WIDTH_CHOICE_8> {
    const PTR_WIDTH: PtrWidth = PTR_WIDTH_8;
}
impl PtrChoiceWidth for PtrChoiceToWidth<PTR_WIDTH_CHOICE_S> {
    const PTR_WIDTH: PtrWidth = PTR_WIDTH_8; //@TODO conditional compilation
}

//struct S<const I: usize, const J: usize> where [(); I+J]:, {}
struct S<const I: usize, const J: usize> {}
trait T {}
impl<const I: usize, const J: usize> T for S<I, J> where [[(); I]; J]: {}

//type PtrBytes<WC> = [u8; PtrChoiceToWidth::< WC >::PTR_WIDTH];
//
//type PtrBytes<const WC: PtrWidthChoice> = [u8; PtrChoiceToWidth::<{ WC }>::PTR_WIDTH];

pub struct BPtr<const WC: PtrWidthChoice> {
    //bytes: [u8; WC.PTR_WIDTH]

    //bytes: [u8; Self::PW]

    //bytes: [u8; PtrChoiceToWidth::<{ WC }>::PTR_WIDTH]
}
trait TT {
    const PW: usize;
}
impl<const WC: PtrWidthChoice> TT for BPtr<WC> {
    const PW: usize = WC as usize;
}

//-------
/// For sealing traits (use as an argument for method(s) that seal a trait). Intentionally _not_
/// public.
struct Seal;

pub trait PtrWidthIndicator {
    //const PTR_WIDTH: usize;
    type Ptr;

    #[allow(private_interfaces)]
    fn sealed(_: Seal);
}

type Bytes<const N: usize> = [u8; N];

pub struct PtrWidth2 {}
impl PtrWidthIndicator for PtrWidth2 {
    type Ptr = Bytes<2>;
    #[allow(private_interfaces)]
    fn sealed(_: Seal) {}
}

/// Alignment (in bytes).
pub type Alignment = u16;
pub const ALIGN_1: Alignment = 1;
pub const ALIGN_2: Alignment = 2;
//pub struct

/// Generic argument `PWI` (implementing [PtrWidthIndicator]) acts like a `const` generic. This is necessary until @TODO
pub struct Pt<PWI: PtrWidthIndicator, const ALIGN: Alignment> {
    //bytes: [u8; PWI::PTR_WIDTH]
    bytes: <PWI as PtrWidthIndicator>::Ptr,

    _pwi: core::marker::PhantomData<PWI>,
}
impl<PWI: PtrWidthIndicator, const ALIGN: Alignment> Pt<PWI, ALIGN> {}
