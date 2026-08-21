#![no_std]

pub mod idx;

/// Intentionally private.
struct Empty;

pub trait NoClone {
    #[allow(private_interfaces)]
    fn assure_no_clone(&self) -> Empty {
        Empty
    }

    #[allow(private_interfaces)]
    fn assure_no_clone_assoc() -> Empty
    where
        Self: Sized,
    {
        Empty
    }
}
fn _assure_no_clone_is_dyn_compatible(nc: &dyn NoClone) {
    nc.assure_no_clone();
}

unsafe extern "C" {
    fn area_non_clone_cant_be_assured();
}

impl<T: Clone> NoClone for T {
    #[allow(private_interfaces)]
    fn assure_no_clone(&self) -> Empty {
        unsafe {
            area_non_clone_cant_be_assured();
        }
        panic!("Unsupported")
    }

    #[allow(private_interfaces)]
    fn assure_no_clone_assoc() -> Empty
    where
        Self: Sized,
    {
        unsafe {
            area_non_clone_cant_be_assured();
        }
        panic!("Unsupported")
    }
}

#[cfg(test)]
mod tests {
    use crate::NoClone;

    #[test]
    fn without_clone() {
        struct S {}
        impl NoClone for S {}
        let s = S {};

        fn pass_no_clone(nc: &dyn NoClone) -> &dyn NoClone {
            core::hint::black_box(nc)
        }
        pass_no_clone(&s).assure_no_clone();
    }
}
