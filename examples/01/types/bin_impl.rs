use area::address::AddrReprIndicator;
use area::alts::VorBin;
use area::alts::alt_bin::Loadable;
//use area::alts::alt_idx::
use super::{LinkedListNodeBinBased, LinkedListNodeIdxBased};
use core::ops::Deref;

impl<'a, I: 'a, ARI: AddrReprIndicator + 'a> Loadable<'a, ARI>
    for LinkedListNodeIdxBased<'a, I, ARI>
{
    type To = LinkedListNodeBinBased<'a, I, ARI>;

    // Without the leading lifetime 'a for the receiver (&'a self) we had difficulties to implement
    // it (and there would be a lot of `unsafe`).
    //
    // &'a self,
    fn load_from(&self, area: <ARI as AddrReprIndicator>::AreaRef<'a>) -> Self::To {
        let result = LinkedListNodeBinBased::<'_, _, ARI> {
            // Passing in ARI and area to the following allows us to factor transmute out of here =
            // out of the userspace.
            //
            //item_vor: VorBin::from_vor_idx::<ARI>(&self.item_vor, area),
            item_vor: VorBin::from_vor_idx(&self.item_vor),

            prev: self.prev.as_ref().map(|ref_idx| (*ref_idx, area).into()),
            next: self.next.as_ref().map(|ref_idx| (*ref_idx, area).into()),
        };
        result
    }
}

// The primary representation for APIs is Bin-based.
impl<'a, I: 'a + PartialEq, ARI: AddrReprIndicator + 'a> LinkedListNodeBinBased<'a, I, ARI> {
    /// @TODO if the following has receiver with lifetime 'a, that is: &'a self
    ///
    /// then we have lifetime problem/unsafe in a loop in has_item(...)
    fn has_item_check(&self, searched: &I) -> bool {
        self.item_vor.eq(searched)
    }

    /// I _only_ _thought_ that this used to need the receiver to be of lifetime 'a - but it
    /// probably didn't need it:
    ///
    /// &'a self
    fn ref_bin_to_next_own_bin(&self) -> Option<Self> {
        self.next.as_ref().map(|next_ref_bin| next_ref_bin.load())
        /*if let Some(next_ref_bin) = self.next.as_ref() {
            let node_own_bin_based = next_ref_bin.load();
            Some(node_own_bin_based)
        } else {
            None
        }*/
    }

    pub fn has_item(&self, searched: &I) -> bool {
        if self.has_item_check(searched) {
            return true;
        }

        let mut node_own_bin_based_opt = self.ref_bin_to_next_own_bin();

        loop {
            if let Some(node_own_bin_based) = node_own_bin_based_opt.as_ref() {
                if node_own_bin_based.has_item_check(searched) {
                    return true;
                }

                node_own_bin_based_opt = node_own_bin_based.ref_bin_to_next_own_bin();

                /*
                // @TODO create and use a shortcut fn for this unsafe.
                let node_own_bin_based_extended: &'a Self =
                    unsafe { core::mem::transmute(node_own_bin_based) };

                let next_node_own_bin_based_opt =
                    node_own_bin_based_extended.ref_bin_to_next_own_bin();

                node_own_bin_based_opt = next_node_own_bin_based_opt;
                */
            } else {
                return false;
            };
            //node_own_bin_based_opt = next_node_own_bin_based_opt;
        }

        /*
        let mut node_own_bin_based;
        let mut first_iteration = true;
        let mut node_ref_bin_based = self;
        loop {
            if first_iteration {

                first_iteration = false;
            }
            if let Some(next_ref_bin) = node_ref_bin_based.next.as_ref() {
                //node = (*next).deref().load(node_ref);
                let ref_node_idx_based = next_ref_bin.deref();
                // @TOO crea and us a shortcut fn. The ma .area pub(crate) again. Se @TOO n alt_binrs
                let _node_own_bin_based = ref_node_idx_based.load_from(next_ref_bin.area);

                if node_own_bin_based.item_vor.eq(searched) {
                    return true;
                }
                node_ref_bin_based = &node_own_bin_based;
            } else {
                return false;
            }
        }*/
    }
}

#[non_exhaustive]
pub struct Iter<'a, I: 'a + PartialEq, ARI: AddrReprIndicator + 'a> {
    initial_ref: Option<&'a LinkedListNodeBinBased<'a, I, ARI>>,
    subsequent_own: Option<LinkedListNodeBinBased<'a, I, ARI>>,
}

impl<'a, I: 'a + PartialEq, ARI: AddrReprIndicator + 'a> Iterator for Iter<'a, I, ARI> {
    type Item = &'a I;

    fn next(&mut self) -> Option<&'a I> {
        if let Some(initial_ref) = self.initial_ref {
            let item_ref = initial_ref.item_vor.deref();

            self.subsequent_own = initial_ref.ref_bin_to_next_own_bin();
            self.initial_ref = None;
            Some(item_ref)
        } else {
            if let Some(subsequent_own) = self.subsequent_own.as_mut() {
                let item_ref = subsequent_own.item_vor.deref();

                self.subsequent_own = subsequent_own.ref_bin_to_next_own_bin();

                todo!()
                //Some(item_ref)
            } else {
                None
            }
        }
    }
}
impl<'a, I: 'a + PartialEq, ARI: AddrReprIndicator + 'a> LinkedListNodeBinBased<'a, I, ARI> {
    pub fn iter(&'a self) -> Iter<'a, I, ARI> {
        Iter {
            initial_ref: Some(self),
            subsequent_own: None,
        }
    }
}
