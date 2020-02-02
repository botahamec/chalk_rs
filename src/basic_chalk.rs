
use crate::Chalk;

use std::fmt::Binary;
use std::fmt::Octal;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::UpperHex;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Deref;

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
    LightGray    = 37,
    Default      = 39,
    DarkGray     = 90,
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
    LightGray    = 47,
    Default      = 49,
    DarkGray     = 100,
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
	pub bgcolor: BasicBackground,
	pub styles: Vec<BasicStyle>
}

impl Display for BasicChalk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[{};{};{}m", self.fgcolor, self.bgcolor, self.clone().style())
    }
}

impl Deref for BasicChalk {
	type Target = BasicChalk;

	fn deref(&self) -> &Self::Target {
		self
	}
}

impl DerefMut for BasicChalk {
	type Target = BasicChalk;

	fn deref_mut(&self) -> &mut Self::Target {
		self
	}
}

impl Add for BasicChalk {
	type Output = BasicChalk;

	fn add(self, other: Self) -> Self {

		let mut chalk = BasicChalk::new();

		if self.fgcolor == BasicColor::Default {chalk.fgcolor = other.fgcolor;}
		if self.bgcolor == BasicBackground::Default {chalk.bgcolor = other.bgcolor;}
		let mut styles = self.styles.clone();
		styles.append(&mut other.styles);
		chalk.styles = styles;

		chalk
	}
}

impl AddAssign for BasicChalk {
	fn add_assign(&mut self, other: Self) {
		if self.fgcolor == BasicColor::Default {self.fgcolor = other.fgcolor;}
		if self.bgcolor == BasicBackground::Default {self.bgcolor = other.bgcolor;}
		self.styles.append(&mut other.styles)
	}
}

impl Chalk for BasicChalk {}

/**
 * Automatically generates a method to change the color
 */
macro_rules! color_fn {
	($snake: ident, $pascal: ident) => {
		/**
		 * Changes the color
		 */
		pub fn $snake(&mut self) -> &Self {
			self.fgcolor = BasicColor::$pascal;
			self
		}
	};
}

/**
 * Automatically generates a method to change the background color
 */
macro_rules! bg_color_fn {
	($snake: ident, $pascal: ident) => {
		/**
		 * Changes the background color
		 */
		pub fn $snake(&mut self) -> &Self {
			self.bgcolor = BasicBackground::$pascal;
			self
		}
	};
}

/**
 * Sets up an alias for a function
 */
macro_rules! fn_alias {
	($alias: ident, $fn: ident) => {
		/** an alias for gray */
		pub fn $alias(&mut self) -> &Self {self.$fn()}
	};
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
	pub fn reset_style(&mut self) -> &Self {
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
		self.styles = vec![BasicStyle::Hidden];
		self
	}

	/**
	 * Makes blinking text
	 */
	pub fn blink(&mut self) -> &Self {
		self.styles.push(BasicStyle::Blink);
		self
	}

	// foreground colors
	color_fn!(reset_color, Default);
	color_fn!(black, Black);
	color_fn!(red, Red);
	color_fn!(green, Green);
	color_fn!(yellow, Yellow);
	color_fn!(blue, Blue);
	color_fn!(magenta, Magenta);
	color_fn!(cyan, Cyan);
	color_fn!(white, White);
	color_fn!(gray, DarkGray);
	color_fn!(light_red, Red);
	color_fn!(light_green, Green);
	color_fn!(light_yellow, LightYellow);
	color_fn!(light_blue, LightBlue);
	color_fn!(light_magenta, LightMagenta);
	color_fn!(light_cyan, LightCyan);
	color_fn!(light_gray, LightGray);

	// aliases for gray()
	fn_alias!(grey, gray);
	fn_alias!(dark_gray, gray);
	fn_alias!(dark_grey, grey);
	fn_alias!(light_black, gray);

	// background colors
	bg_color_fn!(reset_bg, Default);
	bg_color_fn!(bg_black, Black);
	bg_color_fn!(bg_red, Red);
	bg_color_fn!(bg_green, Green);
	bg_color_fn!(bg_yellow, Yellow);
	bg_color_fn!(bg_blue, Blue);
	bg_color_fn!(bg_magenta, Magenta);
	bg_color_fn!(bg_cyan, Cyan);
	bg_color_fn!(bg_white, White);
	bg_color_fn!(bg_gray, DarkGray);
	bg_color_fn!(bg_light_red, LightRed);
	bg_color_fn!(bg_light_green, LightGreen);
	bg_color_fn!(bg_light_yellow, LightYellow);
	bg_color_fn!(bg_light_blue, LightBlue);
	bg_color_fn!(bg_light_magenta, LightMagenta);
	bg_color_fn!(bg_light_cyan, LightCyan);
	bg_color_fn!(bg_light_gray, LightGray);

	fn_alias!(bg_grey, bg_gray);
	fn_alias!(bg_dark_gray, bg_gray);
	fn_alias!(bg_dark_grey, bg_gray);
	fn_alias!(bg_light_black, bg_gray);

}