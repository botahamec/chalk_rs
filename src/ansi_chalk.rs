use std::convert::From;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnsiColor(u8);

impl AnsiColor {
	#[inline(always)]
	pub const fn from_num(color: u8) -> Self {
		AnsiColor(color)
	}

	#[inline(always)]
	pub const fn as_num(self) -> u8 {
		self.0
	}
}

impl From<u8> for AnsiColor {
	#[inline(always)]
	fn from(color: u8) -> Self {
		AnsiColor(color)
	}
}
