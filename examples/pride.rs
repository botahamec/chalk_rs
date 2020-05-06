extern crate chalk_rs;
use chalk_rs::Chalk;

const LINE_LENGTH: usize = 32;

fn main() {

	let mut chalk = Chalk::new();
	let mut line = |color: u8| chalk.bg_ansi(color).println(&[' '; LINE_LENGTH].iter().collect::<String>());
	let mut print_lines = |colors: &[u8]| {
		for i in 0..colors.len() {
			line(colors[i]);
		}
		println!();
	};

	print_lines(&[50, 207, 15, 207, 50]); // trans flag
	print_lines(&[1, 166, 11, 2, 4, 5]); // gay flag
	print_lines(&[16, 240, 15, 47, 15, 240, 16]); // agender flag
	print_lines(&[53, 131, 135, 15, 175, 203, 52]); // lesbian flag
	print_lines(&[3, 3, 15, 15, 5, 5, 16, 16]); // enby flag
	print_lines(&[125, 125, 5, 5, 4, 4]); // bi flag
	print_lines(&[99, 99, 15, 15, 10, 10]); // genderqueer flag
	print_lines(&[127, 127, 11, 11, 39, 39]); // pansexual flag
	print_lines(&[175, 15, 201, 16, 20]); // genderfluid flag
}