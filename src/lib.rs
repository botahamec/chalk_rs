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
Basic, Ansi, and RGB.

```rust
use chalk_rs::prelude::*;

fn main() {
    let mut ansi = AnsiChalk::new();
    ansi.ansi(56).println(&"Purple-ish");
    let mut rgb = RgbChalk::new();
    rgb.rgb(25, 125, 63).println(&"This color is ugly");
}
```

RGB chalk is able to use ANSI and Basic color. ANSI chalk is able to use basic
colors. However, ANSI chalk cannot use RGB and Basic chalk can't use RGB
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

/** Chalk with 256 color support */
pub mod ansi_chalk;

/** Basic 16 colors */
pub mod basic_chalk;

/** True-color 16 million colors */
pub mod rgb_chalk;

/** Styling for the text */
pub mod style;

/** Some basic imports */
pub mod prelude;

mod utils;

use std::string::ToString;

#[cfg(windows)]
use winapi::{
	um::consoleapi::SetConsoleMode,
	um::consoleapi::GetConsoleMode,
	um::processenv::GetStdHandle,
	um::winbase::STD_OUTPUT_HANDLE,
	shared::minwindef::DWORD,
	um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
};

/** For all Chalks with different color types */
pub trait Chalk: Sized + ToString + Default {

	/**
	 * Creates a Chalk with a black background and a white foreground
	 */
	fn new() -> Self {

		// makes it work on windows
		#[cfg(windows)] unsafe {

			static mut IS_SETUP : bool = false;

			if !IS_SETUP {
				let handle = GetStdHandle(STD_OUTPUT_HANDLE);
				let mut dw_mode : DWORD = 0;
				dw_mode |= GetConsoleMode(handle, &mut dw_mode) as u32;
				dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
				SetConsoleMode(handle, dw_mode);

				println!("Hi");

				IS_SETUP = true;
			}
		}

		Self::default()
	}

	/**
	 * Formats a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```rust
	 * # use chalk_rs::prelude::*;
	 * # fn main() {
	 * # let mut chalk = BasicChalk::new();
	 * chalk.yellow().string(&"this is yellow");
	 * # }
	 * ```
	 */
	fn string(&self, string: &dyn ToString) -> String {
		format!("{}{}\x1b[m", self.to_string(), string.to_string())
	}

	/**
	 * Prints a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```rust
	 * # use chalk_rs::prelude::*;
	 * # fn main() {
	 * # let mut chalk = BasicChalk::new();
	 * chalk.yellow().string(&"this is yellow");
	 * # }
	 * ```
	 */
	fn print(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		print!("{}", output);
		output
	}

	/**
	 * Prints a line using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```rust
	 * # use chalk_rs::prelude::*;
	 * # fn main() {
	 * # let mut chalk = BasicChalk::new();
	 * chalk.string(&"this is yellow");
	 * # }
	 * ```
	 */
	fn println(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}

#[cfg(test)]
mod test {

	use crate::*;
	use basic_chalk::*;
	use ansi_chalk::*;

	#[test]
	fn is_setup() {
		let mut basic = basic_chalk::BasicChalk::new();
		let mut ansi = ansi_chalk::AnsiChalk::new();
		basic.red().println(&"This is red");
		ansi.ansi(56).println(&"56");
	}
}