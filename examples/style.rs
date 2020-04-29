
extern crate chalk_rs;
use chalk_rs::basic_chalk::BasicChalk;
use chalk_rs::Chalk;
use chalk_rs::style::ChalkStyle;

fn main() {
	let mut chalk = BasicChalk::new();
	chalk.println(&"Default");
	chalk.bold().println(&"Bold");
	chalk.reset_style().dim().println(&"Dim");
	chalk.reset_style().underline().println(&"Underline");
	chalk.reset_style().blink().println(&"Blink");
	chalk.reset_style().fast_blink().println(&"Fast Blink");
	chalk.reset_style().inverse().println(&"Inverted");
	chalk.reset_style().hidden().println(&"Hidden");
	chalk.reset_style().double_underline().println(&"Double Underline");
}