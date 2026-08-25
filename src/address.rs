use crate::Area;

/// For sealing [AddrReprIndicator] only. Intentionally _not_ public.
trait AddrReprIndicatorSealBase {}

pub(crate) trait IntoUsize {
    fn into_usize(self) -> usize;
}
impl IntoUsize for u16 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
impl IntoUsize for u32 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
impl IntoUsize for u64 {
    fn into_usize(self) -> usize {
        self as usize
    }
}
impl IntoUsize for usize {
    fn into_usize(self) -> usize {
        self
    }
}

/// Indicate address representation.
///
/// 'static isa not strictly necessary, but then we'd need to restrict [AddrReprIndicator::AreaRef]
/// with an extra bound....
#[allow(private_bounds)]
pub trait AddrReprIndicator: AddrReprIndicatorSealBase + Sized + 'static {
    // @TODO add any traits, like From<...>, or make new traits, if needed
    type Addr: IntoUsize + Copy;

    // @TODO
    /// - This is a reference to [crate::Area], that is, `&'a Area<'a, _ARI>`, for most `Addr*Repr*`
    ///   types.
    /// - This is `()` only for [AddrPtr], where [crate::alts::alt_bin::RefBin] does _not_
    /// need to carry [crate::Area] reference at all.
    ///
    /// This has to be [Copy] - for example, [crate::alts::alt_bin::RefBin::from] need it.
    //type AreaType<'a>;
    //
    // OR:
    //
    type AreaRef<'a>: Into<&'a Area<'a, Self>> + Copy;
    //const AREA_ARR_SIZE: usize = 1;
}

/// Like an enum. However, it can't be a wrapper/struct/enum because of @TODO.
type AddrWidthLabel = char;

/// Indicate address representation: width, and whether it's an index (rather than a
/// pointer/reference). This is _not_ going to be an alias to `[u8; N]` (nor to area's internal
/// representation, nor to anything similar), because by having a dedicated type we prevent
/// accidental mistakes (and we make it more a little more forward compatible once relevant Rust
/// const generic-related features get stabilized @TODO).
///
/// However, it *will* be (hopefully) replaced with a `const` generic, once @TODO is stabilized.
///
/// Not to be instantiated outside of this crate (that's why it's `#[non_exhaustive]`). Actually,
/// never to be instantiated, mmap-ed etc. (Hence [AddrRepr::_never_to_instantiate] - which can't
/// fit into any addressable memory).
///
/// `W` indicates how many bytes.
#[non_exhaustive]
struct AddrRepr<const W: AddrWidthLabel, const IS_IDX: bool> {
    _never_to_instantiate: [u64; usize::MAX],
}

/// 2 bytes (16 bit) address label. Respective to [AddrIdx16].
const ADDR_WIDTH_LABEL_16: AddrWidthLabel = '2';
/// 4 bytes (32 bit) address label. Respective to [AddrIdx32].
const ADDR_WIDTH_LABEL_32: AddrWidthLabel = '4';
/// 8 bytes (64 bit) address label. Respective to [AddrIdx64].
const ADDR_WIDTH_LABEL_64: AddrWidthLabel = '8';
/// [usize[-wide address label. Respective to [AddrIdxS] and [AddrPtr]. _Not_ the same as any other
/// value, even if the width matches. That prevents hardcoding of any platform's address width.
///
/// It also prevents mistakes by using [ADDR_WIDTH_LABEL_S] interchangeably with any other width
/// label. Even if they happen to be of the same address width on any platform. (For example
/// [ADDR_WIDTH_LABEL_64] and [ADDR_WIDTH_LABEL_S] having same width on 64-bit platforms.)
const ADDR_WIDTH_LABEL_S: AddrWidthLabel = 's';

/// 2 bytes (16 bit) address. Respective to [ADDR_WIDTH_LABEL_16].
#[allow(private_interfaces)]
pub type AddrIdx16 = AddrRepr<ADDR_WIDTH_LABEL_16, true>;
/// 4 bytes (32 bit) address. Respective to [ADDR_WIDTH_LABEL_32].
#[allow(private_interfaces)]
pub type AddrIdx32 = AddrRepr<ADDR_WIDTH_LABEL_32, true>;
/// 8 bytes (64 bit) address. Respective to [ADDR_WIDTH_LABEL_64].
#[allow(private_interfaces)]
pub type AddrIdx64 = AddrRepr<ADDR_WIDTH_LABEL_64, true>;

/// [usize]-wide address. Respective to [ADDR_WIDTH_LABEL_S]. _Not_ the same as any other value,
/// even if the width matches. That prevents hardcoding of any platform's address width, or mistakes
/// by using [AddrIdxS] interchangeably with any other address indicator. (Even if they happen to be
/// of the same address width on any platform). See also [ADDR_WIDTH_LABEL_S].
#[allow(private_interfaces)]
pub type AddrIdxS = AddrRepr<ADDR_WIDTH_LABEL_S, true>;

/// [usize]-wide address POINTER/reference (_not_ an Area's index).
#[allow(private_interfaces)]
pub type AddrPtr = AddrRepr<ADDR_WIDTH_LABEL_S, false>;

/// Unfortunately, we can't just have
/// ```ignore
/// impl<const W: char> AddrReprIndicator for AddrRepr<W> {
///    type Addr = Bytes< {as_addr_width(W)} >;
///    // ...
/// }
/// ```
/// or anything similar, because that fails @TODO
mod addr_repr_indicator_impls {
    use super::{
        AddrIdx16, AddrIdx32, AddrIdx64, AddrIdxS, AddrPtr, AddrRepr, AddrReprIndicator,
        AddrReprIndicatorSealBase, AddrWidthLabel,
    };
    use crate::Area;

    // @TODO if we change this, it gets *aligned*
    //type Bytes<const N: usize> = [u8; N];

    impl AddrReprIndicator for AddrIdx16 {
        type Addr = u16; //Bytes<2>;
        type AreaRef<'a> = &'a crate::Area<'a, AddrIdx16>;
    }
    impl AddrReprIndicator for AddrIdx32 {
        type Addr = u32; //Bytes<4>;
        type AreaRef<'a> = &'a crate::Area<'a, AddrIdx32>;
    }
    impl AddrReprIndicator for AddrIdx64 {
        type Addr = u64; //Bytes<8>;
        type AreaRef<'a> = &'a crate::Area<'a, AddrIdx64>;
    }
    impl AddrReprIndicator for AddrIdxS {
        /*#[cfg(target_pointer_width = "16")]
        type Addr = u16; //Bytes<2>;
        #[cfg(target_pointer_width = "32")]
        type Addr = u32; //Bytes<4>;
        #[cfg(target_pointer_width = "64")]
        type Addr = u64; //Bytes<8>;*/
        type Addr = usize;

        type AreaRef<'a> = &'a crate::Area<'a, AddrIdxS>;
    }

    #[derive(Clone, Copy)]
    #[non_exhaustive]
    pub struct AreaRefEmpty;
    impl<'a, ARI: AddrReprIndicator> Into<&'a Area<'a, ARI>> for AreaRefEmpty {
        fn into(self) -> &'a Area<'a, ARI> {
            unreachable!("NOT to be used")
        }
    }

    impl AddrReprIndicator for AddrPtr {
        /*#[cfg(target_pointer_width = "16")]
        type Addr = u16;//Bytes<2>;
        #[cfg(target_pointer_width = "32")]
        type Addr = u32;//Bytes<4>;
        #[cfg(target_pointer_width = "64")]
        type Addr = u64;//Bytes<8>;*/
        type Addr = usize;

        /// The stored address is already a pointer/reference. It doesn't need to be resolved, so it
        /// doesn't need [crate::Area].
        //
        //type AreaRef<'a> = ();
        type AreaRef<'a> = AreaRefEmpty;
    }

    /// Blanket impl for any (even incorrect) labels *is* ok, since 3rd party crates can't implement
    /// [AddrReprIndicator] for [AddrRepr] (because both [AddrReprIndicator] and [AddrRepr]
    /// are defined in this crate).
    impl<const PWI: AddrWidthLabel, const IS_IDX: bool> AddrReprIndicatorSealBase
        for AddrRepr<PWI, IS_IDX>
    {
    }
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
