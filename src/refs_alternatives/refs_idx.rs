use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;

pub struct Ref<'_a, _T, AWI: AddrWidthIndicator> {
    bytes: <AWI as AddrWidthIndicator>::Addr,
    _a: PhantomData<&'_a ()>,
    _r: PhantomData<_T>,
}
