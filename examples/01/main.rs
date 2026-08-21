pub mod idx {
    use area::refs_alternatives::refs_idx as refs;

    #[path = "../types.rs"]
    mod types_lifetimed;

    pub mod types {
        use super::types_lifetimed;
        use area::address::AddrWidthS;

        pub type LinkedListNode<T, AWI = AddrWidthS> =
            types_lifetimed::LinkedListNode<'static, T, AWI>;
    }
}

pub mod bin {
    use area::refs_alternatives::refs_bin as refs;

    #[path = "../types.rs"]
    pub mod types;
}

fn main() {}
