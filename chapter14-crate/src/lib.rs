//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            _ => panic!("Stop mixing the same colors"),
        }
    }
}