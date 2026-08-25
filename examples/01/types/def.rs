// This import has to be relative (so _not_ using `crate::...``). The actual type depends on which
// module this file is loaded for.
use super::alt::{Ref, Vor};

use crate::types::LinkedListNodeIdxBased;
use area::address::{AddrIdxS, AddrReprIndicator};

pub struct LinkedListNode<'a, I, ARI: AddrReprIndicator = AddrIdxS> {
    // @TODO make this a wrapper - either around I, or &I (and in both cases: Deref)
    //
    // \--- <-- GAT ARI:Item<I>
    //
    // - Only RefIdx, and RefBin for ARI being AddrPtr if returned by reference from the Area, have
    //   this always as `I` value.
    // - RefBin (for ARI being AddrIdx* but not AddrPtr) has this as &I, unless I: Copy and small
    //   enough.
    //
    //   \--- <-- GAT ARI:ItemCopy<I>
    //
    //  - OR, simplify: *any* RefBin (regardless of ARI) always has it as &I. Let's have Rust/LLVM
    //    optimize it.\
    //
    //    \- --> then this wrapper type doesn't need to be a GAT in ARI. Instead, it can be a proper
    //    type in alts::*, for example alts::*::Value.
    pub(crate) item_vor: Vor<'a, I>,

    //prev: Option<Ref<'ia, LinkedListNode<'ia, I, ARI>, ARI>>,
    //
    //                      |
    //
    //                      \--  when Ref == RefBin: This _inner_ LinkedListNode = from the Area storage = ARI-based
    //
    //                           it needs to be _not_ Self, but RefIdx-based! That is: crate::idx::types::LinkdListNode
    //
    //                           So: It can be, and _has-to-be_, Idx-based *all_the_time*.
    pub(crate) prev: Option<Ref<'a, LinkedListNodeIdxBased<'a, I, ARI>, ARI>>,
    pub(crate) next: Option<Ref<'a, LinkedListNodeIdxBased<'a, I, ARI>, ARI>>,
}
