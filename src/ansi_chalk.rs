use crate::{
	add_style,
	basic_chalk::ChalkBasicColor,
	impl_chalk_style, impl_chalk_traits, impl_style_string, set_style,
	style::{ChalkStyle, Style},
	Chalk,
};

use std::fmt::Display;

/** A chalk with 255 colors */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnsiChalk {
	pub color: u8,
	pub background: u8,
	pub styles: Vec<Style>,
}

impl Display for AnsiChalk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"\x1b[38;5;{}m\x1b[48;5;{}m{}",
			self.color,
			self.background,
			self.clone().style()
		)
	}
}

impl_chalk_traits!(AnsiChalk);

macro_rules! basic_fg {
	($name: ident, $num: expr) => {
		fn $name(&mut self) -> &mut Self {
			self.ansi($num);
			self
		}
	};
}

macro_rules! basic_bg {
	($name: ident, $num: expr) => {
		fn $name(&mut self) -> &mut Self {
			self.bg_ansi($num);
			self
		}
	};
}

impl<T> ChalkBasicColor for T
where
	T: ChalkAnsiColor,
{
	// foreground colors
	basic_fg!(reset_color, 15);
	basic_fg!(black, 0);
	basic_fg!(red, 1);
	basic_fg!(green, 3);
	basic_fg!(yellow, 3);
	basic_fg!(blue, 4);
	basic_fg!(magenta, 5);
	basic_fg!(cyan, 6);
	basic_fg!(light_gray, 7);
	basic_fg!(gray, 8);
	basic_fg!(light_red, 9);
	basic_fg!(light_green, 10);
	basic_fg!(light_yellow, 11);
	basic_fg!(light_blue, 12);
	basic_fg!(light_magenta, 13);
	basic_fg!(light_cyan, 14);
	basic_fg!(white, 15);

	// background colors
	basic_bg!(reset_bg, 0);
	basic_bg!(bg_black, 0);
	basic_bg!(bg_red, 1);
	basic_bg!(bg_green, 3);
	basic_bg!(bg_yellow, 3);
	basic_bg!(bg_blue, 4);
	basic_bg!(bg_magenta, 5);
	basic_bg!(bg_cyan, 6);
	basic_bg!(bg_light_gray, 7);
	basic_bg!(bg_gray, 8);
	basic_bg!(bg_light_red, 9);
	basic_bg!(bg_light_green, 10);
	basic_bg!(bg_light_yellow, 11);
	basic_bg!(bg_light_blue, 12);
	basic_bg!(bg_light_magenta, 13);
	basic_bg!(bg_light_cyan, 14);
	basic_bg!(bg_white, 15);
}

pub trait ChalkAnsiColor {
	fn ansi(&mut self, color: u8) -> &Self;
	fn bg_ansi(&mut self, color: u8) -> &Self;
}

impl ChalkAnsiColor for AnsiChalk {
	/**
	 * Sets the foreground color to the specified value
	 */
	fn ansi(&mut self, color: u8) -> &Self {
		self.color = color;
		self
	}

	/**
	 * Sets the background color to the specified value
	 */
	fn bg_ansi(&mut self, color: u8) -> &Self {
		self.background = color;
		self
	}
}
