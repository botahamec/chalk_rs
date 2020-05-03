use crate::{
	chalk_trait_fns, enum_default, enum_display, enum_fmt_impl, enum_impls,
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
			ansi_str += format!("\x1b[{}", self.weight).as_str();
		}

		if self.blink != Blink::Default {
			ansi_str += format!("\x1b[{}", self.blink).as_str();
		}

		if self.underline != Underline::Default {
			ansi_str += format!("\x1b[{}", self.underline).as_str();
		}

		if self.italic {
			ansi_str += "\x1b[3";
		}
		if self.hidden {
			ansi_str += "\x1b[8";
		}

		write!(f, "{}", ansi_str)
	}
}

impl StyleMap {
	pub fn new() -> Self {
		Self::default()
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

	pub fn unitalicize(&mut self) -> &mut Self {
		self.italic = false;
		self
	}

	pub fn is_italicized(&self) -> bool {
		self.italic
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
		self.blink = Blink::Default;
		self
	}

	pub fn blink(&mut self) -> &mut Self {
		self.blink = Blink::Slow;
		self
	}

	pub fn fast_blink(&mut self) -> &mut Self {
		self.blink = Blink::Fast;
		self
	}

	pub fn is_blinking(&self) -> bool {
		self.blink != Blink::Default
	}

	pub fn is_slowly_blinking(&self) -> bool {
		self.blink == Blink::Slow
	}

	pub fn is_quickly_blinking(&self) -> bool {
		self.blink == Blink::Fast
	}

	pub fn invert(&mut self) -> &mut Self {
		self.invert = true;
		self
	}

	pub fn uninvert(&mut self) -> &mut Self {
		self.invert = false;
		self
	}

	pub fn is_inverted(&self) -> bool {
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

	pub fn is_hidden(&self) -> bool {
		self.hidden
	}
}

/** A style to be applied to the text */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Style {
	Default = 0,
	Bold = 1,
	Dim = 2,
	Italic = 3,
	Underline = 4,
	Blink = 5,
	FastBlink = 6,
	Invert = 7,
	Hidden = 8,
	DoubleUnderline = 21,
}

enum_impls!(Style);

/** Sets the style to a specific vector */
#[macro_export]
macro_rules! set_style {
	($fn_name: ident, $vec: expr) => {
		/** sets the style */
		fn $fn_name(&mut self) -> &mut Self {
			self.styles = $vec;
			self
		}
	};
}

/** Adds a style to the Chalk */
#[macro_export]
macro_rules! add_style {
	($fn_name: ident, $attribute: ident) => {
		/**
		 * Changes the style
		 */
		fn $fn_name(&mut self) -> &mut Self {
			self.styles.push(Style::$attribute);
			self
		}
	};
}

#[macro_export]
macro_rules! impl_style_string {
	($struct: ident) => {
		impl $struct {
			/**
			 * Creates a string which does all of the style,
			 * Helper function for the Chalk implementation
			 */
			fn style(self) -> String {
				let mut style_command = String::with_capacity(12);
				for style in self.styles {
					style_command = format!("{}\x1b[{}m", style_command, style);
				}
				style_command
			}
		}
	};
}

pub trait ChalkStyle {
	chalk_trait_fns!(
		reset_style,
		hidden,
		bold,
		dim,
		italic,
		underline,
		inverse,
		blink,
		fast_blink,
		double_underline
	);
}

#[macro_export]
macro_rules! impl_chalk_style {
	($struct : ident) => {
		impl ChalkStyle for $struct {
			// default and hidden styles
			set_style!(reset_style, vec![Style::Default]);
			set_style!(hidden, vec![Style::Hidden]);

			// styling
			add_style!(bold, Bold);
			add_style!(dim, Dim);
			add_style!(italic, Italic);
			add_style!(underline, Underline);
			add_style!(inverse, Invert);
			add_style!(blink, Blink);
			add_style!(fast_blink, FastBlink);
			add_style!(double_underline, DoubleUnderline);
		}
	};
}
