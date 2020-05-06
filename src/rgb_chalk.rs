/*
const BASIC_COLORS: [RgbColor; 16] = [
	RgbColor {
		red: 0,
		green: 0,
		blue: 0,
	},
	RgbColor {
		red: 128,
		green: 0,
		blue: 0,
	},
	RgbColor {
		red: 0,
		green: 128,
		blue: 0,
	},
	RgbColor {
		red: 128,
		green: 128,
		blue: 0,
	},
	RgbColor {
		red: 0,
		green: 0,
		blue: 128,
	},
	RgbColor {
		red: 128,
		green: 0,
		blue: 128,
	},
	RgbColor {
		red: 0,
		green: 128,
		blue: 128,
	},
	RgbColor {
		red: 192,
		green: 192,
		blue: 192,
	},
	RgbColor {
		red: 128,
		green: 128,
		blue: 128,
	},
	RgbColor {
		red: 255,
		green: 0,
		blue: 0,
	},
	RgbColor {
		red: 0,
		green: 255,
		blue: 0,
	},
	RgbColor {
		red: 255,
		green: 255,
		blue: 0,
	},
	RgbColor {
		red: 0,
		green: 0,
		blue: 255,
	},
	RgbColor {
		red: 255,
		green: 0,
		blue: 255,
	},
	RgbColor {
		red: 0,
		green: 255,
		blue: 255,
	},
	RgbColor {
		red: 255,
		green: 255,
		blue: 255,
	},
];
*/

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RgbColor {
	red: u8,
	green: u8,
	blue: u8,
}

impl RgbColor {

	#[inline(always)]
	pub const fn new(red: u8, green: u8, blue: u8) -> Self {
		RgbColor { red, green, blue }
	}

	#[inline(always)]
	pub const fn get_red(self) -> u8 {
		self.red
	}

	#[inline(always)]
	pub const fn get_green(self) -> u8 {
		self.green
	}

	#[inline(always)]
	pub const fn get_blue(self) -> u8 {
		self.blue
	}
}
