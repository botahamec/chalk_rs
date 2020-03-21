use crate::{
	add_style,
	basic_chalk::ChalkBasicColor,
	ansi_chalk::ChalkAnsiColor,
	set_style,
	style::{ChalkStyle, Style},
	Chalk,
};

use std::fmt::Display;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RgbColor {
	pub red: u8,
	pub green: u8,
	pub blue: u8
}

/** A chalk with RGB colors */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RgbChalk {
	pub color: RgbColor,
	pub background: RgbColor,
	pub styles: Vec<Style>,
}

impl Default for RgbChalk {
	fn default() -> Self {
		RgbChalk {
			color: RgbColor::new(255, 255, 255),
			background: RgbColor::default(),
			styles: Vec::new()
		}
	}
}

impl RgbChalk {

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

impl Display for RgbChalk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"\x1b[38;2;{};{};{};\x1b[48;2;{};{};{};\x1b[{}m",
			self.color.red,
			self.color.green,
			self.color.blue,
			self.background.red,
			self.background.green,
			self.background.blue,
			self.clone().style()
		)
	}
}

impl ChalkStyle for RgbChalk {
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

impl Chalk for RgbChalk {}

trait ChalkRgbColor {
	fn rgb(&mut self, red: u8, green: u8, blue: u8) -> &Self;
	fn bg_rgb(&mut self, red: u8, green: u8, blue: u8) -> &Self;
}

impl RgbColor {
	fn new(red: u8, green: u8, blue: u8) -> Self {
		RgbColor {red, green, blue}
	}
}

impl ChalkRgbColor for RgbChalk {
	fn rgb(&mut self, red: u8, green: u8, blue: u8) -> &Self {
		self.color.red = red;
		self.color.green = green;
		self.color.blue = blue;

		self
	}

	fn bg_rgb(&mut self, red: u8, green: u8, blue: u8) -> &Self {
		self.background.red = red;
		self.background.green = green;
		self.background.blue = blue;

		self
	}
}

impl ChalkAnsiColor for RgbChalk {

	// TODO: modify this so values under 16 are basic colors
	fn ansi(&mut self, color: u8) -> &Self {

		if color > 231 {
			let s = (color - 232) * 10 + 8;
			self.rgb(s, s, s)
		} else {
			let n = color - 16;
			let mut blue = color % 6;
			let mut green = (n - blue) / 6 % 6;
			let mut red = (n - blue - green * 6) / 36 % 6;

			if blue != 0 {
				blue = blue * 40 + 55;
			}
			if green != 0 {
				green = green * 40 + 55;
			}
			if red != 0 {
				red = red * 40 + 55;
			}

			self.rgb(red, green, blue)
		}
	}

	// TODO: modify this so values under 16 are basic colors
	fn bg_ansi(&mut self, color: u8) -> &Self {

		if color > 231 {
			let s = (color - 232) * 10 + 8;
			self.bg_rgb(s, s, s)
		} else {
			let n = color - 16;
			let mut blue = color % 6;
			let mut green = (n - blue) / 6 % 6;
			let mut red = (n - blue - green * 6) / 36 % 6;

			if blue != 0 {
				blue = blue * 40 + 55;
			}
			if green != 0 {
				green = green * 40 + 55;
			}
			if red != 0 {
				red = red * 40 + 55;
			}

			self.bg_rgb(red, green, blue)
		}
	}
}