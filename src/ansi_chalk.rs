
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AnsiColor {
	color: u8
}

impl AnsiColor {

	pub const fn from_num(color: u8) -> Self {
		AnsiColor {color}
	}

	pub const fn as_num(self) -> u8 {
		self.color
	}
}
