//pub trait Referrable {}

//-----------
/*
/** Pointer width choice. As if an enum. */
pub type AddrWidthChoice = u8;
pub const ADDR_WIDTH_CHOICE_2: AddrWidthChoice = 0;
pub const ADDR_WIDTH_CHOICE_4: AddrWidthChoice = 1;
pub const ADDR_WIDTH_CHOICE_8: AddrWidthChoice = 2;
/// `usize`-like
pub const ADDR_WIDTH_CHOICE_S: AddrWidthChoice = 3;

/** Pointer width, in bytes. */
pub type AddrWidth = usize;
pub const ADDR_WIDTH_2: AddrWidth = 2;
pub const ADDR_WIDTH_4: AddrWidth = 4;
pub const ADDR_WIDTH_8: AddrWidth = 8;

pub type Align = usize;
pub type CacheLineWidth = usize;

trait AddrChoiceWidth {
    const ADDR_WIDTH: AddrWidth;
}

struct AddrChoiceToWidth<const C: AddrWidthChoice> {}
impl AddrChoiceWidth for AddrChoiceToWidth<ADDR_WIDTH_CHOICE_2> {
    const ADDR_WIDTH: AddrWidth = ADDR_WIDTH_2;
}
impl AddrChoiceWidth for AddrChoiceToWidth<ADDR_WIDTH_CHOICE_4> {
    const ADDR_WIDTH: AddrWidth = ADDR_WIDTH_4;
}
impl AddrChoiceWidth for AddrChoiceToWidth<ADDR_WIDTH_CHOICE_8> {
    const ADDR_WIDTH: AddrWidth = ADDR_WIDTH_8;
}
impl AddrChoiceWidth for AddrChoiceToWidth<ADDR_WIDTH_CHOICE_S> {
    const ADDR_WIDTH: AddrWidth = ADDR_WIDTH_8; //@TODO conditional compilation
}

//struct S<const I: usize, const J: usize> where [(); I+J]:, {}
struct S<const I: usize, const J: usize> {}
trait T {}
impl<const I: usize, const J: usize> T for S<I, J> where [[(); I]; J]: {}

//type AddrBytes<WC> = [u8; AddrChoiceToWidth::< WC >::ADDR_WIDTH];
//
//type AddrBytes<const WC: AddrWidthChoice> = [u8; AddrChoiceToWidth::<{ WC }>::ADDR_WIDTH];

pub struct BAddr<const WC: AddrWidthChoice> {
    //bytes: [u8; WC.ADDR_WIDTH]

    //bytes: [u8; Self::PW]

    //bytes: [u8; AddrChoiceToWidth::<{ WC }>::ADDR_WIDTH]
}
trait TT {
    const PW: usize;
}
impl<const WC: AddrWidthChoice> TT for BAddr<WC> {
    const PW: usize = WC as usize;
}
*/
//-------

// @TODO remove if unused
//
// For sealing traits (use as an argument for method(s) that seal a trait). Intentionally _not_
// public.
//
//struct Seal;

/// For sealing [AddrWidthIndicator] only. Intentionally _not_ public.
trait AddrWidthIndicatorSealBase {}

#[allow(private_bounds)]
pub trait AddrWidthIndicator: AddrWidthIndicatorSealBase {
    // @TODO add any traits, like From<...>, or make new traits, if needed
    type Addr;
}

/// Like an enum. However, it can't be a wrapper/struct/enum because of @TODO.
pub type AddrWidthLabel = char;

/// Indicate pointer width. This is _not_ going to be an alias to `[u8; N]` (nor to area's internal
/// `Bytes<N>`, nor to anything similar), because by having a dedicated type we prevent accidental
/// mistakes (and we make it more a little more forward compatible once relevant Rust const
/// generic-related features get stabilized @TODO).
///
/// However, it *will* be (hopefully) replaced with a `const` generic, once @TODO is stabilized.
///
/// Not to be instantiated outside of this crate (that's why it's `#[non_exhaustive]`). Actually,
/// never to be instantiated, mmap-ed etc. (Hence [AddrWidth::_never_to_exist] - which can't fit into
/// any addressable memory).
///
/// `W` indicates how many bytes.
#[non_exhaustive]
pub struct AddrWidth<const W: AddrWidthLabel> {
    _never_to_instantiate: [u64; usize::MAX],
}

/// 2 bytes (16 bit) pointer label. Respective to [AddrWidth2].
pub const ADDR_WIDTH_LABEL_2: AddrWidthLabel = '2';
/// 4 bytes (32 bit) pointer label. Respective to [AddrWidth4].
pub const ADDR_WIDTH_LABEL_4: AddrWidthLabel = '4';
/// 8 bytes (64 bit) pointer label. Respective to [AddrWidth8].
pub const ADDR_WIDTH_LABEL_8: AddrWidthLabel = '8';
/// [usize[-wide pointer label. Respective to [AddrWidthS]. _Not_ the same as any other value, even
/// if the width matches. That prevents hardcoding of any platform's pointer width, or mistakes by
/// using [ADDR_WIDTH_LABEL_8] interchangeably with any other width label (even if they happen to be
/// of the same pointer width on any platform).
pub const ADDR_WIDTH_LABEL_S: AddrWidthLabel = 's';

/// 2 bytes (16 bit) pointer. Respective to [ADDR_WIDTH_LABEL_2].
pub type AddrWidth2 = AddrWidth<ADDR_WIDTH_LABEL_2>;
/// 4 bytes (32 bit) pointer. Respective to [ADDR_WIDTH_LABEL_4].
pub type AddrWidth4 = AddrWidth<ADDR_WIDTH_LABEL_4>;
/// 8 bytes (64 bit) pointer. Respective to [ADDR_WIDTH_LABEL_8].
pub type AddrWidth8 = AddrWidth<ADDR_WIDTH_LABEL_8>;
/// [usize]-wide pointer. Respective to [ADDR_WIDTH_LABEL_S]. _Not_ the same as any other value, even
/// if the width matches. That prevents hardcoding of any platform's pointer width, or mistakes by
/// using [AddrWidthS] interchangeably with any other width (even if they happen to be of the same
/// pointer width on any platform).
pub type AddrWidthS = AddrWidth<ADDR_WIDTH_LABEL_S>;

/// Unfortunately, we can't just have
/// ```ignore
/// impl<const W: char> AddrWidthIndicator for AddrWidth<W> {
///    type Addr = Bytes< {as_addr_width(W)} >;
///    // ...
/// }
/// ```
/// or anything similar, because that fails @TODO
mod addr_width_indicator_impls {
    use super::{
        AddrWidth, AddrWidth2, AddrWidth4, AddrWidth8, AddrWidthIndicator,
        AddrWidthIndicatorSealBase, AddrWidthLabel, AddrWidthS,
    };

    type Bytes<const N: usize> = [u8; N];

    impl AddrWidthIndicator for AddrWidth2 {
        type Addr = Bytes<2>;
    }
    impl AddrWidthIndicator for AddrWidth4 {
        type Addr = Bytes<4>;
    }
    impl AddrWidthIndicator for AddrWidth8 {
        type Addr = Bytes<8>;
    }
    impl AddrWidthIndicator for AddrWidthS {
        #[cfg(target_pointer_width = "16")]
        type Addr = Bytes<2>;
        #[cfg(target_pointer_width = "32")]
        type Addr = Bytes<4>;
        #[cfg(target_pointer_width = "64")]
        type Addr = Bytes<8>;
    }

    /// Blanket impl for any (even incorrect) labels *is* ok, since 3rd party crates can't implement
    /// [AddrWidthIndicator] for [AddrWidth] (because both [AddrWidthIndicator] and [AddrWidth]
    /// are defined in this crate).
    impl<const PWI: AddrWidthLabel> AddrWidthIndicatorSealBase for AddrWidth<PWI> {}
}

// @TODO u16 or wrapper?
pub const fn as_addr_width(label: AddrWidthLabel) -> usize {
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

/// @TODO make it a const trait, once Rust stabilizes that. For now use standalone function
/// [as_addr_width] instead (in `const` context).
pub trait AsAddrWidth {
    fn as_addr_width(&self) -> usize;
}
impl AsAddrWidth for AddrWidthLabel {
    fn as_addr_width(&self) -> usize {
        as_addr_width(*self)
    }
}

/// Generic argument `PWI` (implementing [AddrWidthIndicator]) acts like a `const` generic. This is
/// necessary until @TODO
pub struct Pt<T, AWI: AddrWidthIndicator> {
    //bytes: [u8; PWI::ADDR_WIDTH]
    bytes: <AWI as AddrWidthIndicator>::Addr,

    _pwi: core::marker::PhantomData<AWI>,
    _t: core::marker::PhantomData<T>,
}
impl<T, PWI: AddrWidthIndicator> Pt<T, PWI> {
    // @TODO consider removing ALIGN; AND:
    //
    // Do we need Alignment = u16? And/or, have a new wrapper around u16.
    pub const fn alignment(&self) -> usize {
        core::mem::align_of::<T>()
    }
}
//-----

// ---------
// @TODO remove:

/*trait CharToWidth {
    const W: usize = unreachable!();

    // Can't have default for associated types:
    //
    //type Addr2 = ();

    //type Addr2;
}

pub struct AddrWidth<const W: AddrWidthLabel> {}
impl<const WW: AddrWidthLabel> CharToWidth for AddrWidth<WW> {
    const W: usize = const { if true { 0 } else { unreachable!() } };

    // Can't have the following - it fails with error:
    //
    // generic `Self` types are currently not permitted in anonymous constants
    //
    //type Addr2 = [u8; Self::W]
}

impl AddrWidthIndicator for AddrWidth<'2'> {
    type Addr = Bytes<2>;
    #[allow(private_interfaces)]
    fn sealed(_: Seal) {}
}*/

// @TODO examples/docs:
/*
pub struct PtConGen<const PWI: AddrWidthLabel, const ALIGN: Alignment> {
    //bytes: <PWI as AddrWidth>::Addr,

    //bytes:

    //_pwi: core::marker::PhantomData<PWI>,
}
impl<const PWI: AddrWidthLabel, const ALIGN: Alignment> PtConGen<PWI, ALIGN> {}
*/
