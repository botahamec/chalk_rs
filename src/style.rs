use crate::{
	enum_default, enum_display, enum_fmt_impl, enum_impls, impl_enums,
};

use std::fmt::Binary;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Octal;
use std::fmt::UpperHex;

#[cfg(serde)]
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Weight {
	Default,
	Bold = 1,
	Dim = 2,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Underline {
	Default,
	Single = 4,
	Double = 21,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StyleMap {
	weight: Weight,
	underline: Underline,
	italic: bool,
	blink: bool,
	invert: bool,
	hidden: bool
}

impl_enums!(Weight, Underline);

impl Display for StyleMap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut ansi_str = String::with_capacity(3);

		// initially set it to the weight if there is one
		if self.weight != Weight::Default {
			ansi_str += format!("\x1b[{}m", self.weight).as_str();
		}

		if self.underline != Underline::Default {
			ansi_str += format!("\x1b[{}m", self.underline).as_str();
		}

		if self.italic {
			ansi_str += "\x1b[3m";
		}
		if self.blink {
			ansi_str += "\x1b[6m"
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
	pub fn reset_style(&mut self) -> &mut Self {
		self.reset_weight();
		self.stop_blink();
		self.no_underline();
		self.unitalic();
		self.uninvert();
		self.unhide();
		self
	}

	pub fn reset_weight(&mut self) -> &mut Self {
		self.weight = Weight::Default;
		self
	}

	pub fn bold(&mut self) -> &mut Self {
		self.weight = Weight::Bold;
		self
	}

	pub fn dim(&mut self) -> &mut Self {
		self.weight = Weight::Dim;
		self
	}

	pub fn is_normal_weight(&self) -> bool {
		self.weight == Weight::Default
	}

	pub fn is_bold(&self) -> bool {
		self.weight == Weight::Bold
	}

	pub fn is_dim(&self) -> bool {
		self.weight == Weight::Dim
	}

	pub fn italic(&mut self) -> &mut Self {
		self.italic = true;
		self
	}

	pub fn unitalic(&mut self) -> &mut Self {
		self.italic = false;
		self
	}

	pub const fn is_italicized(&self) -> bool {
		self.italic
	}

	pub fn no_underline(&mut self) -> &mut Self {
		self.underline = Underline::Default;
		self
	}

	pub fn underline(&mut self) -> &mut Self {
		self.underline = Underline::Single;
		self
	}

	pub fn double_underline(&mut self) -> &mut Self {
		self.underline = Underline::Double;
		self
	}

	pub fn num_underlines(&self) -> u8 {
		match self.underline {
			Underline::Default => 0,
			Underline::Single => 1,
			Underline::Double => 2,
		}
	}

	pub fn has_underlines(&self) -> bool {
		self.underline != Underline::Default
	}

	pub fn is_single_underlined(&self) -> bool {
		self.underline == Underline::Single
	}

	pub fn is_double_underlined(&self) -> bool {
		self.underline == Underline::Double
	}

	pub fn stop_blink(&mut self) -> &mut Self {
		self.blink = false;
		self
	}

	pub fn blink(&mut self) -> &mut Self {
		self.blink = true;
		self
	}

	pub const fn is_blinking(&self) -> bool {
		self.blink
	}

	pub fn invert(&mut self) -> &mut Self {
		self.invert = true;
		self
	}

	pub fn uninvert(&mut self) -> &mut Self {
		self.invert = false;
		self
	}

	pub const fn is_inverted(&self) -> bool {
		self.invert
	}

	pub fn hide(&mut self) -> &mut Self {
		self.hidden = true;
		self
	}

	pub fn unhide(&mut self) -> &mut Self {
		self.hidden = false;
		self
	}

	pub const fn is_hidden(&self) -> bool {
		self.hidden
	}
}
