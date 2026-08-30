use area::address::AddrReprIndicator;
pub use def_loader::{bin::def as bin, idx::def as idx};

pub use bin::LinkedListNode as LinkedListNodeBinBased;
pub use idx::LinkedListNode as LinkedListNodeIdxBased;

mod bin_impl;
mod def_loader;

/// Reference representation to be used in userland API.
pub type LinkedListNodeRef<'a, 'i, I, ARI> = <ARI as AddrReprIndicator<'a>>::RefData<
    LinkedListNodeIdxBased<'a, 'i, I, ARI>,
    LinkedListNodeBinBased<'a, 'i, I, ARI>,
>;
