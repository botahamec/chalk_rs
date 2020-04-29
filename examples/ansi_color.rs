
extern crate chalk_rs;
use chalk_rs::ansi_chalk::AnsiChalk;
use chalk_rs::Chalk;
use chalk_rs::basic_chalk::ChalkBasicColor;
use chalk_rs::ansi_chalk::ChalkAnsiColor;

fn main() {
	let mut chalk = AnsiChalk::new();

	// foreground colors
	for i in 0..=255 {
		chalk.ansi(i).print(&format!("{} ", i));
		if i < 10 {print!(" ");}
		if i < 100 {print!(" ");}
		if (i as i16 + 1) % 16 == 0 {println!();}
	}

	chalk.reset_color();
	println!();

	for i in 0..=255 {
		chalk.bg_ansi(i).print(&format!("{} ", i));
		if i < 10 {print!(" ");}
		if i < 100 {print!(" ");}
		if (i as i16 + 1) % 16 == 0 {println!();}
	}
}