//! Test that `#[expect(...)]` attributes work with `#[derive_aliases::derive(...)]`
//!
//! Run with: cargo clippy --all-targets
#![allow(unused)]

mod derive_alias {
    derive_aliases::define! {
        Clone = ::core::clone::Clone;
    }
}

#[expect(clippy::large_enum_variant)]
#[derive_aliases::derive(..Clone)]
enum LargeWithAlias {
    Small(u8),
    Big([u8; 1024]),
}

#[expect(clippy::large_enum_variant)]
#[derive_aliases::derive(Clone)]
enum LargeWithoutAlias {
    Small(u8),
    Big([u8; 1024]),
}
