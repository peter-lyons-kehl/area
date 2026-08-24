pub use loader::{bin::def as bin, idx::def as idx};

pub use bin::LinkedListNode as LinkedListNodeBinBased;
pub use idx::LinkedListNode as LinkedListNodeIdxBased;

mod loader;
