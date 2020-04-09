use crate::{
	chalk_trait_fns, enum_default, enum_display, enum_fmt_impl, enum_impls,
};

use std::fmt::Binary;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Octal;
use std::fmt::UpperHex;

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
					style_command = format!("{}{};", style_command, style);
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
			add_style!(double_underline, DoubleUnderline);
		}
	};
}
