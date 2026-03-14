//! Test that derive macros which reference `self` in generated code
//! work correctly through derive_aliases::derive.

mod derive_alias {
    derive_aliases::define! {
        GetValue = ::test_derive::GetValue;
    }
}

// Through an alias
#[derive_aliases::derive(..GetValue)]
struct Aliased(u32);

// Without an alias
#[derive_aliases::derive(test_derive::GetValue)]
struct Direct(u32);

#[test]
fn aliased_derive_can_reference_self() {
    assert_eq!(Aliased(42).value(), 42);
}

#[test]
fn direct_derive_can_reference_self() {
    assert_eq!(Direct(42).value(), 42);
}
