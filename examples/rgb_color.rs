
extern crate chalk_rs;
use chalk_rs::prelude::*;

use std::convert::TryInto;

fn main() {

	let line_length = 78;

	// foreground colors
	let mut chalk = RgbChalk::new();
	for i in 0..line_length {
		let r : u16 = 255 - (i * 255 / line_length);
		let mut g : u16 = i * 510 / line_length;
		let b : u16 = i * 255 / line_length;
		if g > 255 {g = 510 - g;}
		chalk.rgb(
			r.try_into().unwrap(),
			g.try_into().unwrap(),
			b.try_into().unwrap(),
		);
		if i % 3 == 0 {chalk.print(&'r');}
		if i % 3 == 1 {chalk.print(&'g');}
		if i % 3 == 2 {chalk.print(&'b');}
	}

	println!();

	// background color
	chalk.reset_color();
	for i in 0..line_length {
		let r : u16 = 255 - (i * 255 / line_length);
		let mut g : u16 = i * 510 / line_length;
		let b : u16 = i * 255 / line_length;
		if g > 255 {g = 510 - g;}
		chalk.bg_rgb(
			r.try_into().unwrap(),
			g.try_into().unwrap(),
			b.try_into().unwrap(),
		);
		if i % 3 == 0 {chalk.print(&'r');}
		if i % 3 == 1 {chalk.print(&'g');}
		if i % 3 == 2 {chalk.print(&'b');}
	}

	println!();
}