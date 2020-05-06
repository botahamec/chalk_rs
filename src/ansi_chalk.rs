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
