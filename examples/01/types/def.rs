// This import has to be relative (so _not_ using `crate::...``). The actual type depends on which
// module this file is loaded for.
use super::alt::{Ref, VoR};

use crate::types::bin::LinkedListNode as LinkedListNodeBinBased;
use crate::types::idx::LinkedListNode as LinkedListNodeIdxBased;

use area::address::{AddrIdxWidthS, AddrReprIndicator};

pub struct LinkedListNode<'a, 'i: 'a, I, AWI: AddrReprIndicator = AddrIdxWidthS> {
    // @TODO make this a wrapper - either around I, or &I (and in both cases: Deref)
    //
    // \--- <-- GAT AWI:Item<I>
    //
    // - Only RefIdx, and RefBin for AWI being AddrPtrWidthS if returned by reference from the Area,
    //   have this always as `I` value.
    // - RefBin (for AWI being AddrIdxWidth*) has this as &I, unless I: Copy and small enough.
    //
    //   \--- <-- GAT AWI:ItemCopy<I>
    //
    //  - OR, simplify: *any* RefBin (regardless of AWI) always has it as &I. Let's have Rust/LLVM
    //    optimize it.\
    //
    //    \- --> then this wrapper type doesn't need to be a GAT in AWI. Instead, it can be a proper
    //    type in alts::*, for example alts::*::Value.
    item: VoR<'i, I>,

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
