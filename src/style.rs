use crate::{
	enum_default, enum_display, enum_fmt_impl, enum_impls,
	impl_enums,
};

use std::fmt::Binary;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Octal;
use std::fmt::UpperHex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Weight {
	Default,
	Bold = 1,
	Dim = 2,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Blink {
	Default,
	Slow = 5,
	Fast = 6,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Underline {
	Default,
	Single = 4,
	Double = 21,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StyleMap {
	weight: Weight,
	blink: Blink,
	underline: Underline,
	italic: bool,
	invert: bool,
	hidden: bool,
}

impl_enums!(Weight, Blink, Underline);

impl Display for StyleMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ansi_str = String::with_capacity(3);

		// initially set it to the weight if there is one
		if self.weight != Weight::Default {
			ansi_str += format!("\x1b[{}m", self.weight).as_str();
		}

		if self.blink != Blink::Default {
			ansi_str += format!("\x1b[{}m", self.blink).as_str();
		}

		if self.underline != Underline::Default {
			ansi_str += format!("\x1b[{}m", self.underline).as_str();
		}

		if self.italic {
			ansi_str += "\x1b[3m";
		}
		if self.invert {
			ansi_str += "\x1b[7m";
		}
		if self.hidden {
			ansi_str += "\x1b[8m";
		}

		write!(f, "{}", ansi_str)
	}
}

impl StyleMap {
	pub const fn new() -> Self {
		Self::default()
	}

	pub const fn reset_style(&mut self) {
		self.reset_weight();
		self.stop_blink();
		self.stop_underline();
		self.unitalicize();
		self.uninvert();
		self.unhide();
	}

	pub const fn reset_weight(&mut self) {
		self.weight = Weight::Default;
	}

	pub const fn bold(&mut self) {
		self.weight = Weight::Bold;
	}

	pub const fn dim(&mut self) {
		self.weight = Weight::Dim;
	}

	pub const fn is_normal_weight(&self) -> bool {
		self.weight == Weight::Default
	}

	pub const fn is_bold(&self) -> bool {
		self.weight == Weight::Bold
	}

	pub const fn is_dim(&self) -> bool {
		self.weight == Weight::Dim
	}

	pub const fn italic(&mut self) {
		self.italic = true;
	}

	pub const fn unitalicize(&mut self) {
		self.italic = false;
	}

	pub const fn is_italicized(&self) -> bool {
		self.italic
	}

	pub const fn stop_underline(&mut self) {
		self.underline = Underline::Default;
	}

	pub const fn underline(&mut self) {
		self.underline = Underline::Single;
	}

	pub const fn double_underline(&mut self) {
		self.underline = Underline::Double;
	}

	pub fn num_underlines(&self) -> u8 {
		match self.underline {
			Underline::Default => 0,
			Underline::Single => 1,
			Underline::Double => 2,
		}
	}

	pub const fn has_underlines(&self) -> bool {
		self.underline != Underline::Default
	}

	pub const fn is_single_underlined(&self) -> bool {
		self.underline == Underline::Single
	}

	pub const fn is_double_underlined(&self) -> bool {
		self.underline == Underline::Double
	}

	pub const fn stop_blink(&mut self) {
		self.blink = Blink::Default;
	}

	pub const fn blink(&mut self) {
		self.blink = Blink::Slow;
	}

	pub const fn fast_blink(&mut self) {
		self.blink = Blink::Fast;
	}

	pub const fn is_blinking(&self) -> bool {
		self.blink != Blink::Default
	}

	pub const fn is_slowly_blinking(&self) -> bool {
		self.blink == Blink::Slow
	}

	pub const fn is_quickly_blinking(&self) -> bool {
		self.blink == Blink::Fast
	}

	pub const fn invert(&mut self) {
		self.invert = true;
	}

	pub const fn uninvert(&mut self) {
		self.invert = false;
	}

	pub const fn is_inverted(&self) -> bool {
		self.invert
	}

	pub const fn hide(&mut self) {
		self.hidden = true;
	}

	pub const fn unhide(&mut self) {
		self.hidden = false;
	}

	pub const fn is_hidden(&self) -> bool {
		self.hidden
	}
}
