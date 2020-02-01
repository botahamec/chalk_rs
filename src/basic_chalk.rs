
use crate::Chalk;

use std::fmt::Binary;
use std::fmt::Octal;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::UpperHex;

/**
 * Implements Default for an enum.
 * Requires the enum to have a variant named "Default"
 */
macro_rules! enum_default {
	($name: ident) => {
		impl Default for $name {fn default() -> Self {$name::Default}}
	}
}

/**
 * Implements a fmt trait for an enum.
 * The output is the enum as a number
 */
macro_rules! enum_fmt_impl {
	($name: ident, $trait: ident) => {
		impl $trait for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		        $trait::fmt(&(self.clone() as u8), f)
		    }
		}
	}
}

/**
 * Implements the Display trait for an enum.
 * The output is the enum as a number
 */
macro_rules! enum_display {
	($name: ident) => {
		impl Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		        write!(f, "{}", self.clone() as u8)
		    }
		}
	}
}

/**
 * Implements several traits for a macro
 */
macro_rules! enum_impls {
	($name: ident) => {
		enum_default!($name);
		enum_display!($name);
		enum_fmt_impl!($name, Binary);
		enum_fmt_impl!($name, Octal);
		enum_fmt_impl!($name, LowerHex);
		enum_fmt_impl!($name, UpperHex);
	};
}

/**
 * A style to be applied to the text
 */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicStyle {
    Default   = 0,
    Bold      = 1,
	Dim       = 2,
    Underline = 4,
    Blink     = 5,
    Invert    = 7,
    Hidden    = 8
}

enum_impls!(BasicStyle);

/**
 * Foreground color using basic color
 */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicColor {
    Black        = 30,
    Red          = 31,
    Green        = 32,
    Yellow       = 33,
    Blue         = 34,
    Magenta      = 35,
    Cyan         = 36,
    LightGrey    = 37,
    Default      = 39,
    DarkGrey     = 90,
    LightRed     = 91,
    LightGreen   = 92,
    LightYellow  = 93,
    LightBlue    = 94,
    LightMagenta = 95,
    LightCyan    = 96,
    White        = 97
}

enum_impls!(BasicColor);

/**
 * The background of a teerminal using basic color
 */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicBackground {
    Black        = 40,
    Red          = 41,
    Green        = 42,
    Yellow       = 43,
    Blue         = 44,
    Magenta      = 45,
    Cyan         = 46,
    LightGrey    = 47,
    Default      = 49,
    DarkGrey     = 100,
    LightRed     = 101,
    LightGreen   = 102,
    LightYellow  = 103,
    LightBlue    = 104,
    LightMagenta = 105,
    LightCyan    = 106,
    White        = 107
}

enum_impls!(BasicBackground);

/**
 * A chalk with only 16 colors
 */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BasicChalk {
	pub fgcolor: BasicColor,
	pub bgcolor: BasicColor,
	pub styles: Vec<BasicStyle>
}

impl Display for BasicChalk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{};{}m", self.fgcolor, self.bgcolor, self.clone().style())
    }
}

impl BasicChalk {

	/**
	 * Returns a new BasicChalk.
	 * This has all default styling.
	 */
	pub fn new() -> Self {Self::default()}

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

	/**
	 * Resets the styling to the default
	 */
	pub fn reset(&mut self) -> &Self {
		self.styles = vec![BasicStyle::Default];
		self
	}

	/**
	 * Makes the text bold
	 */
	pub fn bold(&mut self) -> &Self {
		self.styles.push(BasicStyle::Bold);
		self
	}

	/**
	 * Makes the text dim
	 */
	pub fn dim(&mut self) -> &Self {
		self.styles.push(BasicStyle::Dim);
		self
	}

	/**
	 * Underlines the text
	 */
	pub fn underline(&mut self) -> &Self {
		self.styles.push(BasicStyle::Underline);
		self
	}

	/**
	 * Invert the foreground and background colors
	 */
	pub fn inverse(&mut self) -> &Self {
		self.styles.push(BasicStyle::Invert);
		self
	}

	/**
	 * Makes the text invisible, but copy-pastable
	 */
	pub fn hidden(&mut self) -> &Self {
		self.styles.push(BasicStyle::Hidden);
		self
	}

	/**
	 * Makes blinking text
	 */
	pub fn blink(&mut self) -> &Self {
		self.styles.push(BasicStyle::Blink);
		self
	}
}

impl Chalk for BasicChalk {}