/*!
A crate for terminal colors and styles

```rust
use chalk_rs::prelude::*;
fn main() {
	let mut chalk = BasicChalk::new();
	chalk.red().println(&"This text is red");
	chalk.bold().println(&"Now it's red AND bold");
}
```

That's an example of basic color. There are three types of color in chalk:
BasicChalk, AnsiChalk, and RgbChalk.

```rust
use chalk_rs::prelude::*;

fn main() {
	let mut ansi = AnsiChalk::new();
	ansi.ansi(56).println(&"Purple-ish");
	let mut rgb = RgbChalk::new();
	rgb.rgb(25, 125, 63).println(&"This color is ugly");
}
```

RgbChalk is able to use ANSI and Basic color. AnsiChalk is able to use basic
colors. However, AnsiChal cannot use RGB and BasicChalk can't use RGB
or ANSI.

```rust
use chalk_rs::prelude::*;

fn main() {
	let mut rgb = RgbChalk::new();
	rgb.ansi(56).println(&"Purple-ish");
	rgb.red().println(&"red");
}
```
*/

#![allow(clippy::tabs_in_doc_comments)]

mod ansi_chalk;
mod basic_chalk;
mod rgb_chalk;
mod style;
mod utils;

use basic_chalk::BasicColor;
use ansi_chalk::AnsiColor;
use rgb_chalk::RgbColor;
use style::StyleMap;

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
	DefaultColor,
	BasicColor(BasicColor),
	AnsiColor(AnsiColor),
	RgbColor(RgbColor)
}

impl ChalkType {

	#[inline(always)]
	pub const fn default() -> Self {
		Self::DefaultColor
	}

	#[inline(always)]
	pub const fn black() -> Self {
		Self::BasicColor(BasicColor::Black)
	}

	#[inline(always)]
	pub const fn red() -> Self {
		Self::BasicColor(BasicColor::Red)
	}

	#[inline(always)]
	pub const fn green() -> Self {
		Self::BasicColor(BasicColor::Green)
	}

	#[inline(always)]
	pub const fn yellow() -> Self {
		Self::BasicColor(BasicColor::Yellow)
	}

	#[inline(always)]
	pub const fn blue() -> Self {
		Self::BasicColor(BasicColor::Blue)
	}

	#[inline(always)]
	pub const fn magenta() -> Self {
		Self::BasicColor(BasicColor::Magenta)
	}

	#[inline(always)]
	pub const fn cyan() -> Self {
		Self::BasicColor(BasicColor::Cyan)
	}

	#[inline(always)]
	pub const fn light_gray() -> Self {
		Self::BasicColor(BasicColor::LightGray)
	}

	#[inline(always)]
	pub const fn light_grey() -> Self {
		Self::light_gray()
	}

	#[inline(always)]
	pub const fn gray() -> Self {
		Self::BasicColor(BasicColor::Gray)
	}

	#[inline(always)]
	pub const fn grey() -> Self {
		Self::gray()
	}

	#[inline(always)]
	pub const fn light_black() -> Self {
		Self::gray()
	}

	#[inline(always)]
	pub const fn light_red() -> Self {
		Self::BasicColor(BasicColor::LightRed)
	}

	#[inline(always)]
	pub const fn light_green() -> Self {
		Self::BasicColor(BasicColor::LightGreen)
	}

	#[inline(always)]
	pub const fn light_yellow() -> Self {
		Self::BasicColor(BasicColor::LightYellow)
	}

	#[inline(always)]
	pub const fn light_blue() -> Self {
		Self::BasicColor(BasicColor::LightBlue)
	}

	#[inline(always)]
	pub const fn light_magenta() -> Self {
		Self::BasicColor(BasicColor::LightMagenta)
	}

	#[inline(always)]
	pub const fn light_cyan() -> Self {
		Self::BasicColor(BasicColor::LightCyan)
	}

	#[inline(always)]
	pub const fn white() -> Self {
		Self::BasicColor(BasicColor::White)
	}

	#[inline(always)]
	pub const fn ansi(color: u8) -> Self {
		Self::AnsiColor(AnsiColor::from_num(color))
	}

	#[inline(always)]
	pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
		Self::RgbColor(RgbColor::new(r, g, b))
	}
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chalk {
	foreground: ChalkType,
	background: ChalkType,
	style: StyleMap
}

/** For all Chalks with different color types */
impl Chalk {
	/// Creates a [`Chalk`] with a black background and a white foreground
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::prelude::*;
	///
	/// fn main() {
	///     let mut chalk = BasicChalk::new();
	///     // the chalk can be used here
	/// }
	/// ```
	fn new() -> Self {
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

		Self {
			foreground: ChalkType::DefaultColor,
			background: ChalkType::DefaultColor,
			style: StyleMap::new()
		}
	}

	/// Formats a string using the style of the given [`Chalk`].
	///
	/// This will return the string after being formatted to the console. When
	/// using string literals, please use a reference.
	///
	/// For example:
	///
	/// ```rust
	/// # use chalk_rs::prelude::*;
	/// #
	/// # fn main() {
	/// #   let mut chalk = BasicChalk::new();
	/// chalk.yellow().string(&"this is yellow");
	/// # }
	/// ```
	/// # Arguments
	///
	/// * `string` - The item to print out. It must implement [`ToString`]
	/// and should be a reference
	///
	/// # Example
	///
	/// ```rust
	/// use chalk_rs::prelude::*;
	///
	/// fn main() {
	///     let mut chalk = BasicChalk::new();
	///     chalk.yellow().string(&"this is yellow");
	/// }
	/// ```
	fn string(&self, string: &dyn ToString) -> String {
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
	/// # use chalk_rs::prelude::*;
	/// # fn main() {
	/// # let mut chalk = BasicChalk::new();
	/// chalk.yellow().print(&"this is yellow");
	/// # }
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
	/// use chalk_rs::prelude::*;
	/// fn main() {
	///     let mut chalk = BasicChalk::new();
	///    chalk.yellow().print(&"this is yellow");
	/// }
	/// ```
	fn print(&self, string: &dyn ToString) -> String {
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
	/// # use chalk_rs::prelude::*;
	/// # fn main() {
	/// # let mut chalk = BasicChalk::new();
	/// chalk.yellow().println(&"this is yellow");
	/// # }
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
	/// use chalk_rs::prelude::*;
	/// fn main() {
	///     let mut chalk = BasicChalk::new();
	///    chalk.yellow().println(&"this is yellow");
	/// }
	/// ```
	fn println(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}

#[cfg(test)]
mod test {

	use crate::*;

	#[test]
	fn is_setup() {
		// TODO fix this test
		let mut basic = Chalk::new();
		basic.red().println(&"This is red");
	}
}
