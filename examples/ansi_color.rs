extern crate chalk_rs;
use chalk_rs::Chalk;

fn main() {
	let mut chalk = Chalk::new();

	// foreground colors
	for i in 0..=255 {
		if i < 10 {
			print!(" ");
		}
		if i < 100 {
			print!(" ");
		}
		chalk.ansi(i).print(&format!("{} ", i));
		if (i as i16 + 1) % 16 == 0 {
			println!();
		}
	}

	chalk.default_color();
	println!();

	for i in 0..=255 {
		chalk.bg_ansi(i);
		if i < 10 {
			chalk.print(&" ");
		}
		if i < 100 {
			chalk.print(&" ");
		}
		chalk.print(&format!("{} ", i));
		if (i as i16 + 1) % 16 == 0 {
			println!();
		}
	}
}
