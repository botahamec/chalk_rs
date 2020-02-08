
pub mod basic_chalk;

mod utils;

use std::string::ToString;

/**
 * For all Chalks with different color types
 */
trait Chalk: Sized + ToString {

	/**
     * Formats a string using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
	fn string(self, string: &dyn ToString) -> String {
		format!("{}{}\x1b[m", self.to_string(), string.to_string())
	}

	/**
     * Prints a string using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
	fn print(self, string: &dyn ToString) -> String {
		let output = self.string(string);
		print!("{}", output);
		output
	}

	/**
     * Prints a line using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
	fn println(self, string: &dyn ToString) -> String {
		let output = self.string(string);
		println!("{}", output);
		output
	}
}