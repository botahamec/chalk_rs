#[test]
fn basic_color() {
	use super::Chalk;
	use super::basic_chalk::BasicChalk;
	use super::basic_chalk::ChalkBasicColor;

	let mut chalk = BasicChalk::new();
	chalk.blue().println(&"This is blue");
	chalk.bg_white().black().println(&"This is black");
}
