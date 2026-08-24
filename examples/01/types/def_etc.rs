//use area::alts::alt_bin::RefBin;
use area::alts::alt_bin::Loadable;
//use area::alts::alt_idx::
use super::{LinkedListNodeBinBased, LinkedListNodeIdxBased};

pub mod idx {
    use area::alts::alt_idx as alt;

    #[path = "../../def.rs"]
    mod def_lifetimed;

    pub mod def {
        use super::def_lifetimed;
        use area::address::AddrIdxS;

        //pub use def_lifetimed::LinkedListNode;
        //
        // \--- instead of repeating all items to export, "automate":
        //
        // --- needed only if we ever add any items here on top/as extra
        pub use super::def_lifetimed::*;

        //pub type LinkedListNodeStatic<I, AWI = AddrIdxS> = LinkedListNode<'static, 'static, I, AWI>;
    }
}

pub mod bin {
    use area::alts::alt_bin as alt;

    #[path = "../../def.rs"]
    pub mod def;
}

/*
impl<'a, 'i: 'a, I, ARI: AddrReprIndicator> Loadable for LinkedListNodeIdxBased<'a, 'i, ARI> {
    type To<'a, 't: 'a, T: 't, ARI: area::address::AddrReprIndicator> = LinkedListNodeBinBased<'a, 'i, ARI>
        where
            Self: 't,
            ARI: 'a;
}
*/
