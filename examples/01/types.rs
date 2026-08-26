pub use def_loader::{bin::def as bin, idx::def as idx};

pub use bin::LinkedListNode as LinkedListNodeBinBased;
pub use idx::LinkedListNode as LinkedListNodeIdxBased;

mod bin_impl;
mod def_loader;
