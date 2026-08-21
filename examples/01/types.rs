// This import has to be relative (so _not_ using `crate::...``):
use super::refs::Ref;

use area::address::{AddrWidthIndicator, AddrWidthS};

struct LinkedListNode<'a, T, AWI: AddrWidthIndicator = AddrWidthS> {
    t: T,
    next: Option<Ref<'a, LinkedListNode<'a, T, AWI>, AWI>>,
}
