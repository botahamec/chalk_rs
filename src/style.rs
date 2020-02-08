use crate::{
	enum_default,
	enum_display,
	enum_fmt_impl,
	enum_impls,
	chalk_trait_fns
};

use std::fmt::Binary;
use std::fmt::Octal;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::UpperHex;

/** A style to be applied to the text */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Style {
    Default         = 0,
    Bold            = 1,
	Dim             = 2,
	Italic          = 3,
    Underline       = 4,
	Blink           = 5,
	FastBlink       = 6,
    Invert          = 7,
	Hidden          = 8,
	DoubleUnderline = 21
}

enum_impls!(Style);

/** Sets the style to a specific vector */
#[macro_export]
macro_rules! set_style {
	($fn_name: ident, $vec: expr) => {
		/** sets the style */
		fn $fn_name(&mut self) -> &Self {
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
		fn $fn_name(&mut self) -> &Self {
			self.styles.push(Style::$attribute);
			self
		}
	};
}

pub trait ChalkStyle {
	chalk_trait_fns!(reset_style, hidden, bold, dim, italic, underline,
					inverse, blink, double_underline);
}