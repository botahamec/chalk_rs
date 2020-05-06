/*!
A crate for terminal colors and styles

```rust
use chalk_rs::Chalk;

let mut chalk = Chalk::new();
chalk.red().println(&"This text is red");
chalk.bold().println(&"Now it's red AND bold");
```

That's an example of basic color. There are three types of color in chalk:
BasicChalk, AnsiChalk, and RgbChalk.

```rust
use chalk_rs::Chalk;


let mut chalk = Chalk::new();
chalk.ansi(56).println(&"Purple-ish");
chalk.rgb(25, 125, 63).println(&"This color is ugly");
```

Chalk can aldo do *styling*! Here's an example:

```rust
use chalk_rs::Chalk;

let mut chalk = Chalk::new();
chalk.bold().println(&"Bold!");
```
*/

#![allow(clippy::tabs_in_doc_comments)]

mod ansi_chalk;
mod basic_chalk;
mod rgb_chalk;
mod style;
mod utils;

use ansi_chalk::AnsiColor;
use basic_chalk::BasicColor;
use rgb_chalk::RgbColor;
use style::StyleMap;

use std::fmt::Display;
use std::string::ToString;

#[cfg(windows)]
use winapi::{
	shared::minwindef::DWORD, um::consoleapi::GetConsoleMode,
	um::consoleapi::SetConsoleMode, um::processenv::GetStdHandle,
	um::winbase::STD_OUTPUT_HANDLE,
	um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ChalkType {
	Default,
	Basic(BasicColor),
	Ansi(AnsiColor),
	Rgb(RgbColor),
}

macro_rules! basic_color {
	($fn_name: ident, $color: ident) => {
		#[inline(always)]
		pub const fn $fn_name() -> Self {
			Self::Basic(BasicColor::$color)
		}
	};
}

macro_rules! basic_alias {
	($alias: ident, $func: ident) => {
		#[inline(always)]
		pub const fn $alias() -> Self {
			Self::$func()
		}
	};
}

impl ChalkType {
	#[inline(always)]
	pub const fn default() -> Self {
		Self::Default
	}

	basic_color!(black, Black);
	basic_color!(red, Red);
	basic_color!(green, Green);
	basic_color!(yellow, Yellow);
	basic_color!(blue, Blue);
	basic_color!(magenta, Magenta);
	basic_color!(cyan, Cyan);
	basic_color!(light_gray, LightGray);
	basic_color!(gray, Gray);
	basic_color!(light_red, LightRed);
	basic_color!(light_green, LightGreen);
	basic_color!(light_yellow, LightYellow);
	basic_color!(light_blue, LightBlue);
	basic_color!(light_magenta, LightMagenta);
	basic_color!(light_cyan, LightCyan);
	basic_color!(white, White);

	basic_alias!(light_grey, light_gray);
	basic_alias!(grey, gray);
	basic_alias!(light_black, gray);

	#[inline(always)]
	pub const fn ansi(color: u8) -> Self {
		Self::Ansi(AnsiColor::from_num(color))
	}

	#[inline(always)]
	pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self::Rgb(RgbColor::new(r, g, b))
	}
}

impl Default for ChalkType {
	#[inline(always)]
	fn default() -> Self {
		Self::default()
	}
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chalk {
	foreground: ChalkType,
	background: ChalkType,
	style: StyleMap,
}

impl Chalk {
	#[inline(always)]
	fn foreground_to_string(&self) -> String {
		match &self.foreground {
			ChalkType::Default => String::new(),
			ChalkType::Basic(c) => format!("\x1b[{}m", c.as_foreground_color()),
			ChalkType::Ansi(c) => format!("\x1b[38;5;{}m", c.as_num()),
			ChalkType::Rgb(c) => format!(
				"\x1b[38;2;{};{};{}m",
				c.get_red(),
				c.get_green(),
				c.get_blue()
			),
		}
	}

	#[inline(always)]
	fn background_to_string(&self) -> String {
		match &self.background {
			ChalkType::Default => String::new(),
			ChalkType::Basic(c) => format!("\x1b[{}m", c.as_background_color()),
			ChalkType::Ansi(c) => format!("\x1b[48;5;{}m", c.as_num()),
			ChalkType::Rgb(c) => format!(
				"\x1b[48;2;{};{};{}m",
				c.get_red(),
				c.get_green(),
				c.get_blue()
			),
		}
	}
}

impl Display for Chalk {
	fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut string = String::with_capacity(9);
		string += self.foreground_to_string().as_str();
		string += self.background_to_string().as_str();
		string += self.style.to_string().as_str();
		write!(fmt, "{}", string)
	}
}

/** For all Chalks with different color types */
impl Chalk {
	/// Creates a [`Chalk`] with a black background and a white foreground
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::Chalk;
	///
	/// let mut chalk = Chalk::new();
	/// // the chalk can be used here
	/// ```
	pub fn new() -> Self {
		// makes it work on windows
		#[cfg(windows)]
		unsafe {
			static mut IS_SETUP: bool = false;

			if !IS_SETUP {
				let handle = GetStdHandle(STD_OUTPUT_HANDLE);
				let mut dw_mode: DWORD = 0;
				dw_mode |= GetConsoleMode(handle, &mut dw_mode) as u32;
				dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
				SetConsoleMode(handle, dw_mode);

				println!("Hi");

				IS_SETUP = true;
			}
		}

		Self::default()
	}

	/// Formats a string using the style of the given [`Chalk`].
	///
	/// This will return the string after being formatted to the console. When
	/// using string literals, please use a reference.
	///
	/// For example:
	///
	/// ```rust
	/// # use chalk_rs::Chalk;
	/// # let mut chalk = Chalk::new();
	/// chalk.yellow().string(&"this is yellow");
	/// ```
	/// # Arguments
	///
	/// * `string` - The item to print out. It must implement [`ToString`]
	/// and should be a reference
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::Chalk;
	///
	/// let mut chalk = Chalk::new();
	/// let text = chalk.yellow().string(&"this is yellow");
	/// ```
	pub fn string(&self, string: &dyn ToString) -> String {
		format!("{}{}\x1b[m", self.to_string(), string.to_string())
	}

	/// Prints a string using the style of the given chalk.
	///
	/// This will return the text that was outputted to the console. When using
	/// string literals, please use a reference.
	///
	/// For example:
	///
	/// ```rust
	/// # use chalk_rs::Chalk;
	/// # let mut chalk = Chalk::new();
	/// chalk.yellow().print(&"this is yellow");
	/// ```
	///
	/// # Arguments
	///
	/// * `string` - The item to format and output. It must implement [`ToString`]
	/// and should be a reference.
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::Chalk;
	///
	/// let mut chalk = Chalk::new();
	/// chalk.yellow().print(&"this is yellow");
	/// ```
	pub fn print(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		print!("{}", output);
		output
	}

	/// Prints a string using the style of the given chalk on a new line.
	///
	/// This will return the text that was outputted to the console. When using
	/// string literals, please use a reference.
	///
	/// For example:
	///
	/// ```rust
	/// # use chalk_rs::Chalk;
	/// # let mut chalk = Chalk::new();
	/// chalk.yellow().println(&"this is yellow");
	/// ```
	///
	/// # Arguments
	///
	/// * `string` - The item to format and output. It must implement [`ToString`]
	/// and should be a reference.
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::Chalk;
	///
	/// let mut chalk = Chalk::new();
	/// chalk.yellow().println(&"this is yellow");
	/// ```
	pub fn println(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}

macro_rules! color_fg {
	($fn_name: ident) => {
		#[inline(always)]
		pub fn $fn_name(&mut self) -> &mut Self {
			self.foreground = ChalkType::$fn_name();
			self
		}
	};
}

impl Chalk {
	#[inline(always)]
	pub fn default_color(&mut self) -> &mut Self {
		self.foreground = ChalkType::default();
		self
	}

	color_fg!(black);
	color_fg!(red);
	color_fg!(green);
	color_fg!(yellow);
	color_fg!(blue);
	color_fg!(magenta);
	color_fg!(cyan);
	color_fg!(light_gray);
	color_fg!(light_grey);
	color_fg!(gray);
	color_fg!(grey);
	color_fg!(light_black);
	color_fg!(light_red);
	color_fg!(light_green);
	color_fg!(light_yellow);
	color_fg!(light_blue);
	color_fg!(light_magenta);
	color_fg!(light_cyan);
	color_fg!(white);

	#[inline(always)]
	pub fn ansi(&mut self, color: u8) -> &mut Self {
		self.foreground = ChalkType::ansi(color);
		self
	}

	#[inline(always)]
	pub fn rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
		self.foreground = ChalkType::rgb(r, g, b);
		self
	}
}

macro_rules! color_bg {
	($fn_name: ident, $color: ident) => {
		#[inline(always)]
		pub fn $fn_name(&mut self) -> &mut Self {
			self.background = ChalkType::$color();
			self
		}
	};
}

impl Chalk {
	color_bg!(default_background, default);
	color_bg!(bg_black, black);
	color_bg!(bg_red, red);
	color_bg!(bg_green, green);
	color_bg!(bg_yellow, yellow);
	color_bg!(bg_blue, blue);
	color_bg!(bg_magenta, magenta);
	color_bg!(bg_cyan, cyan);
	color_bg!(bg_light_gray, light_gray);
	color_bg!(bg_light_grey, light_grey);
	color_bg!(bg_gray, gray);
	color_bg!(bg_grey, grey);
	color_bg!(bg_light_black, light_black);
	color_bg!(bg_light_red, light_red);
	color_bg!(bg_light_green, light_green);
	color_bg!(bg_light_yellow, light_yellow);
	color_bg!(bg_light_blue, light_blue);
	color_bg!(bg_light_magenta, light_magenta);
	color_bg!(bg_light_cyan, light_cyan);
	color_bg!(bg_white, white);

	#[inline(always)]
	pub fn bg_ansi(&mut self, color: u8) -> &mut Self {
		self.background = ChalkType::ansi(color);
		self
	}

	#[inline(always)]
	pub fn bg_rgb(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
		self.background = ChalkType::rgb(r, g, b);
		self
	}
}

macro_rules! set_style {
	($fn_name: ident) => {
		#[inline(always)]
		pub fn $fn_name(&mut self) -> &mut Self {
			self.style.$fn_name();
			self
		}
	};
}

macro_rules! check_style {
	($fn_name: ident) => {
		#[inline(always)]
		pub fn $fn_name(&self) -> bool {
			self.style.$fn_name()
		}
	};
}

impl Chalk {
	set_style!(reset_style);
	set_style!(reset_weight);
	set_style!(bold);
	set_style!(dim);
	set_style!(italic);
	set_style!(unitalic);
	set_style!(no_underline);
	set_style!(underline);
	set_style!(double_underline);
	set_style!(stop_blink);
	set_style!(blink);
	set_style!(invert);
	set_style!(uninvert);
	set_style!(hide);
	set_style!(unhide);

	check_style!(is_normal_weight);
	check_style!(is_bold);
	check_style!(is_dim);
	check_style!(is_italicized);
	check_style!(has_underlines);
	check_style!(is_single_underlined);
	check_style!(is_double_underlined);
	check_style!(is_blinking);
	check_style!(is_inverted);
	check_style!(is_hidden);

	pub fn num_underlines(&self) -> u8 {
		self.style.num_underlines()
	}
}

#[cfg(test)]
mod test {

	use crate::*;

	#[test]
	fn is_setup() {
		Chalk::new().red().println(&"This is red");
		Chalk::new().blue().println(&"This is blue");
	}
}
