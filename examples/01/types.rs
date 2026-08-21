// This import has to be relative (so _not_ using `crate::...``). The actual type depends on which
// module this file is loaded for.
use super::refs::Ref;

use area::address::{AddrWidthIndicator, AddrWidthS};

pub struct LinkedListNode<'a, T, AWI: AddrWidthIndicator = AddrWidthS> {
    t: T,
    next: Option<Ref<'a, LinkedListNode<'a, T, AWI>, AWI>>,
}
