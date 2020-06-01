//use std::convert::TryFrom;

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

/*
impl TryFrom<u8> for BasicColor {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			30 => Ok(BasicColor::Black),
			31 => Ok(BasicColor::Red),
			32 => Ok(BasicColor::Green),
			33 => Ok(BasicColor::Yellow),
			34 => Ok(BasicColor::Blue),
			35 => Ok(BasicColor::Magenta),
			36 => Ok(BasicColor::Cyan),
			37 => Ok(BasicColor::LightGray),
			90 => Ok(BasicColor::Gray),
			91 => Ok(BasicColor::LightRed),
			92 => Ok(BasicColor::LightGreen),
			93 => Ok(BasicColor::LightYellow),
			94 => Ok(BasicColor::LightBlue),
			95 => Ok(BasicColor::LightMagenta),
			96 => Ok(BasicColor::LightCyan),
			97 => Ok(BasicColor::White),
			_ => Err(())
		}
	}
}*/