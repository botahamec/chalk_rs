#![allow(clippy::tabs_in_doc_comments)]

// TODO: make a prelude module

pub mod ansi_chalk;
pub mod basic_chalk;
pub mod rgb_chalk;
pub mod style;

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

/**
 * For all Chalks with different color types
 */
pub trait Chalk: Sized + ToString + Default {
	fn new() -> Self {
		if cfg!(windows) {
			unsafe {
				let handle = GetStdHandle(STD_OUTPUT_HANDLE);
				let mut dw_mode : DWORD = 0;
				dw_mode |= GetConsoleMode(handle, &mut dw_mode) as u32;
				dw_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
				SetConsoleMode(handle, dw_mode);
			}
		}
		Self::default()
	}

	/**
	 * Formats a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```ignore
	 * chalk.string(&"this is yellow");
	 * ```
	 */
	fn string(&self, string: &dyn ToString) -> String {
		format!("{}{}\x1b[m", self.to_string(), string.to_string())
	}

	/**
	 * Prints a string using the style of the given chalk.
	 * When using string literals, please use a reference.
	 * For example:
	 * ```ignore
	 * chalk.string(&"this is yellow");
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
	 * ```ignore
	 * chalk.string(&"this is yellow");
	 * ```
	 */
	fn println(&self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}
