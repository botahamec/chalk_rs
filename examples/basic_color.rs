
extern crate chalk_rs;
use chalk_rs::basic_chalk::BasicChalk;
use chalk_rs::basic_chalk::ChalkBasicColor;
use chalk_rs::Chalk;

fn main() {

	// basic colors
	let mut chalk = BasicChalk::new();
	chalk.bg_white().black().println(&"This is black");
	chalk.reset_bg().red().println(&"This is red");
	chalk.green().println(&"This is green");
	chalk.yellow().println(&"This is yellow");
	chalk.blue().println(&"This is blue");
	chalk.magenta().println(&"This is magenta");
	chalk.white().println(&"This is white");
	chalk.grey().println(&"This is grey");

	println!();

	// basic light colors
	chalk.light_red().println(&"This is light red");
	chalk.light_green().println(&"This is light green");
	chalk.light_yellow().println(&"This is light yellow");
	chalk.light_blue().println(&"This is light blue");
	chalk.light_magenta().println(&"This is light magenta");
	chalk.light_black().println(&"This is light black");
	chalk.light_grey().println(&"This is light grey");

	println!();

	// basic backgrounds
	chalk.reset_color().bg_red().println(&"This is red");
	chalk.bg_green().println(&"This is green");
	chalk.bg_yellow().println(&"This is yellow");
	chalk.bg_blue().println(&"This is blue");
	chalk.bg_magenta().println(&"This is magenta");
	chalk.black().bg_white().println(&"This is white");
	chalk.reset_color().bg_grey().println(&"This is grey");

	println!();

	// basic light backgrounds
	chalk.bg_light_red().println(&"This is light red");
	chalk.bg_light_green().println(&"This is light green");
	chalk.bg_light_yellow().println(&"This is light yellow");
	chalk.bg_light_blue().println(&"This is light blue");
	chalk.bg_light_magenta().println(&"This is light magenta");
	chalk.bg_light_black().println(&"This is light black");
	chalk.bg_light_grey().println(&"This is light grey");
}