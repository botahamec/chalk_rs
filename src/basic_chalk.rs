/** Foreground color using basic color */
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BasicColor {
	Black = 30,
	Red = 31,
	Green = 32,
	Yellow = 33,
	Blue = 34,
	Magenta = 35,
	Cyan = 36,
	LightGray = 37,
	Gray = 90,
	LightRed = 91,
	LightGreen = 92,
	LightYellow = 93,
	LightBlue = 94,
	LightMagenta = 95,
	LightCyan = 96,
	White = 97,
}

impl BasicColor {
	#[inline(always)]
	pub const fn as_foreground_color(self) -> u8 {
		self as u8
	}

	#[inline(always)]
	pub const fn as_background_color(self) -> u8 {
		self as u8 + 10
	}
}
