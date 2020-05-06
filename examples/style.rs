extern crate chalk_rs;
use chalk_rs::Chalk;

fn main() {
	let mut chalk = Chalk::new();
	chalk.println(&"Default");
	chalk.bold().println(&"Bold");
	chalk.reset_style().dim().println(&"Dim");
	chalk.reset_style().italic().println(&"Italics");
	chalk.reset_style().underline().println(&"Underline");
	chalk.reset_style().blink().println(&"Blink");
	chalk.reset_style().invert().println(&"Inverted");
	chalk.reset_style().hide().println(&"Hidden");
	chalk
		.reset_style()
		.double_underline()
		.println(&"Double Underline");
	chalk
		.reset_style()
		.strikethrough()
		.println(&"Strikethrough");
}
