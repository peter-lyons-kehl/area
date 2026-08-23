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
        pub use super::def_lifetimed::*;

        pub type LinkedListNodeStatic<I, AWI = AddrIdxS> = LinkedListNode<'static, 'static, I, AWI>;
    }
}

pub mod bin {
    use area::alts::alt_bin as alt;

    #[path = "../../def.rs"]
    pub mod def;
}
