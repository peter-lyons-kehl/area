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

impl<'a, I: 'a, ARI: AddrReprIndicator + 'a> Loadable<'a, ARI>
    for LinkedListNodeIdxBased<'a, I, ARI>
// @TODO why do we need the following bound?
//
//where
//    Self: 'i,
{
    type To = LinkedListNodeBinBased<'a, I, ARI>;

    // Without the leading lifetime 'a for the receiver (&'a self) we had difficulties to implement it
    //
    // fn load(&self,...
    fn load(&self, _area: <ARI as area::address::AddrReprIndicator>::AreaRef<'a>) -> Self::To {
        //@TODO

        //let _: LinkedListNodeBinBased<'_, _, ARI> = LinkedListNodeBinBased {
        let result = LinkedListNodeBinBased::<'_, _, ARI> {
            //item_vor: VoRBin::new(self.vor.as_ref()),
            item_vor: VoRBin::<'a, I>::new({
                let rf = self.item_vor.as_ref();
                unsafe { core::mem::transmute(rf) }
            }),
            //
            //item_vor: loop {},
            //
            prev: None, //@TODO
            next: None, //@TODO
        };
        if false {
            take_outlives::<'a, _>(result);
            todo!()
        } else {
            result
        }
        //unsafe { core::mem::transmute(result) }
    }
}

trait Outlives<'a>: 'a {}
impl<'a, T: 'a> Outlives<'a> for T {}
fn take_outlives<'a, T: 'a>(_: T) {}
