
extern crate chalk_rs;
use chalk_rs::prelude::*;

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