// This import has to be relative (so _not_ using `crate::...``). The actual type depends on which
// module this file is loaded for.
use super::alt::Ref;

use crate::types::bin::LinkedListNode as LinkedListNodeBinBased;
use crate::types::idx::LinkedListNode as LinkedListNodeIdxBased;

use area::address::{AddrIdxWidthS, AddrWidthIndicator};

pub struct LinkedListNode<'a, 'i: 'a, I, AWI: AddrWidthIndicator = AddrIdxWidthS> {
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
    prev: Option<Ref<'a, 'i, LinkedListNodeIdxBased<'a, 'i, I, AWI>, AWI>>,
    next: Option<Ref<'a, 'i, LinkedListNodeIdxBased<'a, 'i, I, AWI>, AWI>>,
}
