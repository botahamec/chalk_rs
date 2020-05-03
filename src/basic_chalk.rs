use crate::{
	add_style, chalk_trait_fns, enum_default, enum_display, enum_fmt_impl,
	enum_impls, fn_alias, impl_chalk_style, impl_chalk_traits,
	impl_style_string, set_style,
	style::{ChalkStyle, Style},
	Chalk,
};

use std::fmt::Binary;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::Octal;
use std::fmt::UpperHex;

<<<<<<< HEAD
=======
use std::ops::Add;
use std::ops::AddAssign;

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

/** Implements several traits for a macro */
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

/** A style to be applied to the text */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicStyle {
    Default         = 0,
    Bold            = 1,
	Dim             = 2,
	Italic          = 3,
    Underline       = 4,
    Blink           = 5,
    Invert          = 7,
	Hidden          = 8,
	DoubleUnderline = 21
}

enum_impls!(BasicStyle);

/** The styling for the underline */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Underline {
	Default    = 0,
	Underlined = 1,
	Double     = 2,
	Curly      = 3
}

enum_impls!(Underline);

/** The styling for text */
#[derive(Clone, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BasicStyleMap {
	pub bold     : bool,
	pub dim      : bool,
	pub italic   : bool,
	pub blink    : bool,
	pub invert   : bool,
	pub underline: Underline
}

/** describes how to style text */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicStyleType {
	Default,
	Hidden,
	Styled(BasicStyleMap)
}

enum_default!(BasicStyleType); // This is going to need its own display function

impl Display for BasicStyleType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BasicStyleType::Default => write!(f, ""),
			BasicStyleType::Hidden  => write!(f, "8;"),
			BasicStyleType::Styled(s) => {
				let mut style_string = String::new();
				if s.bold      {style_string.push_str("1;");}
				if s.dim       {style_string.push_str("2;");}
				if s.italic    {style_string.push_str("3;");}
				if s.blink     {style_string.push_str("5;");}
				if s.invert    {style_string.push_str("7;");}
				if s.underline != Underline::Default {
					style_string.push_str(format!("4:{};", s.underline.clone() as u8).as_str());
				}
				write!(f, "{}", style_string)
			}
		}
	}
}

>>>>>>> 1c3fdca70bb859218e7b6400fa4fc82b78cb42c3
/** Foreground color using basic color */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BasicColor {
	Black = 30,
	Red = 31,
	Green = 32,
	Yellow = 33,
	Blue = 34,
	Magenta = 35,
	Cyan = 36,
	LightGray = 37,
	Default = 39,
	DarkGray = 90,
	LightRed = 91,
	LightGreen = 92,
	LightYellow = 93,
	LightBlue = 94,
	LightMagenta = 95,
	LightCyan = 96,
	White = 97,
}

enum_impls!(BasicColor);

/** The background of a teerminal using basic color */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum BasicBackground {
	Black = 40,
	Red = 41,
	Green = 42,
	Yellow = 43,
	Blue = 44,
	Magenta = 45,
	Cyan = 46,
	LightGray = 47,
	Default = 49,
	DarkGray = 100,
	LightRed = 101,
	LightGreen = 102,
	LightYellow = 103,
	LightBlue = 104,
	LightMagenta = 105,
	LightCyan = 106,
	White = 107,
}

enum_impls!(BasicBackground);

/** A chalk with only 16 colors */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BasicChalk {
	fgcolor: BasicColor,
	bgcolor: BasicBackground,
	styles: Vec<Style>,
}

impl Display for BasicChalk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"\x1b[{}m\x1b[{}m{}",
			self.fgcolor,
			self.bgcolor,
			self.clone().style()
		)
	}
}

impl_chalk_traits!(BasicChalk);

/** Automatically generates a method to change the color */
macro_rules! color_fn {
	($snake: ident, $pascal: ident) => {
		/** Changes the color */
		fn $snake(&mut self) -> &mut Self {
			self.fgcolor = BasicColor::$pascal;
			self
		}
	};
}

/** Automatically generates a method to change the background color */
macro_rules! bg_color_fn {
	($snake: ident, $pascal: ident) => {
		/** Changes the background color */
		fn $snake(&mut self) -> &mut Self {
			self.bgcolor = BasicBackground::$pascal;
			self
		}
	};
}

macro_rules! gray_aliases {
	($($alias: ident),*) => {
		$(
			/** an alias for the color gray */
			fn_alias!($alias, gray);
		)*
	};
}

macro_rules! bg_gray_aliases {
	($($alias: ident),*) => {
		$(
			/** an alias for the color gray */
			fn_alias!($alias, bg_gray);
		)*
	};
}

pub trait ChalkBasicColor {
	chalk_trait_fns!(
		reset_color,
		black,
		red,
		green,
		yellow,
		blue,
		magenta,
		cyan,
		white,
		gray,
		light_red,
		light_green,
		light_yellow,
		light_blue,
		light_magenta,
		light_cyan,
		light_gray,
		reset_bg,
		bg_black,
		bg_red,
		bg_green,
		bg_yellow,
		bg_blue,
		bg_magenta,
		bg_cyan,
		bg_white,
		bg_gray,
		bg_light_red,
		bg_light_green,
		bg_light_yellow,
		bg_light_blue,
		bg_light_magenta,
		bg_light_cyan,
		bg_light_gray
	);

	fn_alias!(light_grey, light_gray);
	fn_alias!(bg_light_grey, bg_light_gray);
	gray_aliases!(grey, dark_gray, dark_grey, light_black);
	bg_gray_aliases!(bg_grey, bg_dark_gray, bg_dark_grey, bg_light_black);
}

impl ChalkBasicColor for BasicChalk {
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
	color_fn!(light_red, LightRed);
	color_fn!(light_green, LightGreen);
	color_fn!(light_yellow, LightYellow);
	color_fn!(light_blue, LightBlue);
	color_fn!(light_magenta, LightMagenta);
	color_fn!(light_cyan, LightCyan);
	color_fn!(light_gray, LightGray);

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
}
