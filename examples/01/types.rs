// This import has to be relative (so _not_ using `crate::...``). The actual type depends on which
// module this file is loaded for.
use super::refs::Ref;

use crate::idx::types::LinkedListNode as LinkedListNodeIdxBased;

use area::address::{AddrWidthIndicator, AddrWidthS};

pub struct LinkedListNode<'ia, I, AWI: AddrWidthIndicator = AddrWidthS> {
    item: I,

    //prev: Option<Ref<'ia, LinkedListNode<'ia, I, AWI>, AWI>>,
    //
    //                      |
    //
    //                      \--  when Ref == RefBin: This _inner_ LinkedListNode
    //
    //                           needs to be _not_ Self, but RefIdx-based! That is: crate::idx::types::LinkdListNode
    //
    //                           So: It can be Idx-based *all_the_time*.
    prev: Option<Ref<'ia, LinkedListNodeIdxBased<'ia, I, AWI>, AWI>>,

    next: Option<Ref<'ia, LinkedListNodeIdxBased<'ia, I, AWI>, AWI>>,
}
