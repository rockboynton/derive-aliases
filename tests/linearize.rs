#![allow(unused)]
#![feature(macro_metavar_expr)]

mod derive_alias {
    derive_aliases::define! {
        Linearized = ::linearize::Linearize;
        LinearizedDebug = ::linearize::Linearize, ::core::fmt::Debug;
    }
}

// Handle __integration_test feature
derive_aliases::test! { Color
    #[cfg_attr(all(), ::core::prelude::v1::derive(::linearize::Linearize))]
}

#[derive_aliases::derive(..Linearized)]
enum Color {
    Red,
    Green,
    Blue,
}

derive_aliases::test! { Direction
    #[cfg_attr(all(), ::core::prelude::v1::derive(::linearize::Linearize))]
    #[cfg_attr(all(), ::core::prelude::v1::derive(::core::fmt::Debug))]
}

#[derive_aliases::derive(..LinearizedDebug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[test]
fn linearize_via_alias() {
    use linearize::Linearize;
    assert_eq!(Color::Red.linearize(), 0);
    assert_eq!(Color::Green.linearize(), 1);
    assert_eq!(Color::Blue.linearize(), 2);
    assert_eq!(Color::LENGTH, 3);
}

#[test]
fn linearize_mixed_with_std_derives() {
    use linearize::Linearize;
    assert_eq!(Direction::North.linearize(), 0);
    assert_eq!(Direction::West.linearize(), 3);
    assert_eq!(Direction::LENGTH, 4);
    // Debug also works
    assert_eq!(format!("{:?}", Direction::North), "North");
}
