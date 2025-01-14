//! # Crates
//!
//! `crates` is a collection of utilities random tasks.
//! This is being used as a way to learn about crates,
//! and should **not** be uploaded to [crates.io](https://crates.io).

pub mod kinds {
	pub enum PrimaryColor {
		Red,
		Yellow,
		Blue,
	}

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
	pub fn mix(_color1: PrimaryColor, _color2: PrimaryColor) -> SecondaryColor {
		// Just to get this to compile
		SecondaryColor::Green
	}
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}
