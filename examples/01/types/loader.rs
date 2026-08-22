pub mod idx {
    use area::alts::alt_idx as alt;

    #[path = "../../def.rs"]
    mod def_lifetimed;

    pub mod def {
        use super::def_lifetimed;
        use area::address::AddrWidthS;

        pub use def_lifetimed::LinkedListNode;
        pub type LinkedListNodeStatic<I, AWI = AddrWidthS> =
            LinkedListNode<'static, 'static, I, AWI>;
    }
}

pub mod bin {
    use area::alts::alt_bin as alt;

    #[path = "../../def.rs"]
    pub mod def;
}
