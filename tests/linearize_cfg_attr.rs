//! Regression test: derive_aliases::derive combined with cfg_attr(derive(...))
//! on the same item should not cause hygiene issues with derive macros like
//! linearize::Linearize that use quote_spanned!.
#![allow(unused)]

mod derive_alias {
    derive_aliases::define! {
        Linearized = ::linearize::Linearize;
    }
}

// Required by the __integration_test feature: define a no-op macro matching the type name
macro_rules! Season {
    ($($tt:tt)*) => {};
}

#[derive_aliases::derive(..Linearized, ::core::default::Default)]
#[cfg_attr(all(), derive(::core::fmt::Debug))]
#[linearize(const)]
enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[test]
fn linearize_with_cfg_attr_derive() {
    use linearize::Linearize;
    assert_eq!(Season::Spring.linearize(), 0);
    assert_eq!(Season::Winter.linearize(), 3);
    assert_eq!(Season::LENGTH, 4);
    // Debug from cfg_attr also works
    assert_eq!(format!("{:?}", Season::Spring), "Spring");
}
