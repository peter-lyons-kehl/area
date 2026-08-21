use crate::address::AddrWidthIndicator;
use core::marker::PhantomData;

pub struct Ref<'a, T, _AWI: AddrWidthIndicator> {
    r: &'a T,
    _awi: PhantomData<_AWI>,
}
