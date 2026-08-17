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
// @TODO remove if unused
/// For sealing traits (use as an argument for method(s) that seal a trait). Intentionally _not_
/// public.
struct Seal;

/// For sealing [PtrWidthIndicator] only. Intentionally _not_ public.
trait PtrWidthIndicatorBase {}

#[allow(private_bounds)]
pub trait PtrWidthIndicator: PtrWidthIndicatorBase {
    //const PTR_WIDTH: usize;
    type Ptr;
}

type Bytes<const N: usize> = [u8; N];

pub type PtrWidthLabel = char;

/// Not to be instantiated outside of this crate (that's why `#[non_exhaustive]`). Actually, never
/// to be instantiated, mmap-ed etc. (hence [PtrWidthConGen::_never_to_exist] - which
/// can't fit into any addressable RAM).
#[non_exhaustive]
pub struct PtrWidthConGen<const W: PtrWidthLabel> {
    _never_to_exist: [u64; usize::MAX],
}

/// Indicate that pointers use 2 bytes.
///
/// This is _not_ going to be an alias to `[u8; 2]` (nor to area's internal `Bytes<2>`, nor to
/// anything similar), because by having a dedicated type we prevent accidental mistakes (and we
/// make it more forward compatible once relevant unstable Rust features get stabilized @TODO).
///
/// However, it *will* be (hopefully) replaced with a `const` generic, once @TODO is stabilized.
pub type PtrWidth2 = PtrWidthConGen<'2'>;
pub type PtrWidth4 = PtrWidthConGen<'4'>;
pub type PtrWidth8 = PtrWidthConGen<'8'>;
pub type PtrWidthS = PtrWidthConGen<'s'>;
/// Unfortunately, we can't just have
/// ```ignore
/// impl<const W: char> PtrWidthIndicator for PtrWidthConGen<W> {
///    type Ptr = Bytes< {as_ptr_width(W)} >;
///    // ...
/// }
/// ```
/// or anything similar, because that fails @TODO
impl PtrWidthIndicator for PtrWidth2 {
    type Ptr = Bytes<2>;
}
impl PtrWidthIndicator for PtrWidth4 {
    type Ptr = Bytes<4>;
}
impl PtrWidthIndicator for PtrWidth8 {
    type Ptr = Bytes<8>;
}
impl PtrWidthIndicator for PtrWidthS {
    #[cfg(target_pointer_width = "16")]
    type Ptr = Bytes<2>;
    #[cfg(target_pointer_width = "32")]
    type Ptr = Bytes<4>;
    #[cfg(target_pointer_width = "64")]
    type Ptr = Bytes<8>;
}

/// Blanket impl for any (even incorrect) labels *is* ok, since 3rd party crates can't implement
/// [PtrWidthIndicator] for [PtrWidthConGen], because both [PtrWidthIndicator] and [PtrWidthConGen]
/// are defined in this crate.
impl<const W: PtrWidthLabel> PtrWidthIndicatorBase for PtrWidthConGen<W> {}

const fn as_ptr_width(label: PtrWidthLabel) -> usize {
    match label {
        '2' => 2,
        '4' => 4,
        '8' => 8,
        's' => {
            #[cfg(target_pointer_width = "16")]
            let w = 2;
            #[cfg(target_pointer_width = "32")]
            let w = 4;
            #[cfg(target_pointer_width = "64")]
            let w = 8;
            w
        }
        _ => panic!("Unsupported pointer width."),
    }
}

/// @TODO make it a const trait, once Rust stabilizes that. For now use [as_ptr_width] instead (in
/// `const` context).
pub trait AsPtrWidth {
    fn as_ptr_width(&self) -> usize;
}
impl AsPtrWidth for PtrWidthLabel {
    fn as_ptr_width(&self) -> usize {
        as_ptr_width(*self)
    }
}

/// Alignment (in bytes).
pub type Alignment = u16;
pub const ALIGN_1: Alignment = 1;
pub const ALIGN_2: Alignment = 2;
pub const ALIGN_4: Alignment = 4;
pub const ALIGN_8: Alignment = 8;
pub const ALIGN_16: Alignment = 16;
pub const ALIGN_32: Alignment = 32;
pub const ALIGN_64: Alignment = 64;
pub const ALIGN_128: Alignment = 128;

/// Generic argument `PWI` (implementing [PtrWidthIndicator]) acts like a `const` generic. This is necessary until @TODO
pub struct Pt<T, PWI: PtrWidthIndicator, const ALIGN: Alignment> {
    //bytes: [u8; PWI::PTR_WIDTH]
    bytes: <PWI as PtrWidthIndicator>::Ptr,

    _pwi: core::marker::PhantomData<PWI>,
    _t: core::marker::PhantomData<T>,
}
impl<T, PWI: PtrWidthIndicator, const ALIGN: Alignment> Pt<T, PWI, ALIGN> {
    // @TODO consider removing ALIGN; AND: Do we need Alignment = u16? And/or, have a new wrapper around u16.
    pub const fn alignment(&self) -> usize {
        core::mem::align_of::<T>()
    }
}
trait PtCheck {
    // @TODO make this actually read, so that it does get linked
    const CHECK: ();
}
impl<T, PWI: PtrWidthIndicator, const ALIGN: Alignment> PtCheck for Pt<T, PWI, ALIGN> {
    const CHECK: () = {
        let is_power_of_two = ALIGN.is_power_of_two();
        if (ALIGN as usize) < core::mem::align_of::<T>() {
            if is_power_of_two {
                panic!("Insufficient alignment (and not a power of two)");
            } else {
                panic!("Insufficient alignment");
            }
        }
        if ALIGN as usize != core::mem::align_of::<T>() {
            if is_power_of_two {
                panic!("Too high alignment (and not a power of two)");
            } else {
                // Hmm:
                //
                // - https://doc.rust-lang.org/core/mem/fn.align_of.html -> "may be smaller than the
                //   preferred alignment"
                //
                // -
                panic!("Too high  alignment");
            }
        }
    };
}

//-----

// ---------
// @TODO remove:

/*trait CharToWidth {
    const W: usize = unreachable!();

    // Can't have default for associated types:
    //
    //type Ptr2 = ();

    //type Ptr2;
}

pub struct PtrWidthConGen<const W: PtrWidthLabel> {}
impl<const WW: PtrWidthLabel> CharToWidth for PtrWidthConGen<WW> {
    const W: usize = const { if true { 0 } else { unreachable!() } };

    // Can't have the following - it fails with error:
    //
    // generic `Self` types are currently not permitted in anonymous constants
    //
    //type Ptr2 = [u8; Self::W]
}

impl PtrWidthIndicator for PtrWidthConGen<'2'> {
    type Ptr = Bytes<2>;
    #[allow(private_interfaces)]
    fn sealed(_: Seal) {}
}*/

pub struct PtConGen<const PWI: PtrWidthLabel, const ALIGN: Alignment> {
    //bytes: <PWI as PtrWidthConGen>::Ptr,

    //bytes:

    //_pwi: core::marker::PhantomData<PWI>,
}
impl<const PWI: PtrWidthLabel, const ALIGN: Alignment> PtConGen<PWI, ALIGN> {}
