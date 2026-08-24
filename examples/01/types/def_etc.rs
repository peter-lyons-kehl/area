//use area::alts::alt_bin::RefBin;
use area::address::AddrReprIndicator;
use area::alts::VoRBin;
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

impl<'a, 'i: 'a, I, ARI: AddrReprIndicator> Loadable<'a, ARI>
    for LinkedListNodeIdxBased<'a, 'i, I, ARI>
// @TODO why do we need the following bound?
where
    Self: 'i,
{
    type To = LinkedListNodeBinBased<'a, 'i, I, ARI>;

    fn load(&self, _area: <ARI as area::address::AddrReprIndicator>::AreaRef<'a>) -> Self::To {
        /*LinkedListNodeBinBased {
            item: VoRBin::new(&self.item),
            prev: None,
            next: None
        }*/
        todo!()
    }
}
