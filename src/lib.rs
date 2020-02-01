
pub mod basic_chalk;

use std::string::ToString;

/**
 * For all Chalks with different color types
 */
pub trait Chalk {

    /**
     * Formats a string using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
    fn string(self, string: &dyn ToString) -> String;

    /**
     * Prints a string using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
    fn print(self, string: &dyn ToString) -> String;

    /**
     * Prints a line using the style of the given chalk.
     * When using string literals, please use a reference.
     * For example:
     * ```rust
     * chalk.string(&"this is yellow");
     * ```
     */
    fn println(self, string: &dyn ToString) -> String;
}