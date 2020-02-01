
use crate::Chalk;

/**
 * A style to be applied to the text
 */
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum BasicStyle {
    Normal    = 0,
    Bold      = 1,
    Dim       = 2,
    Underline = 4,
    Blink     = 5,
    Invert    = 7,
    Hidden    = 8
}

impl Default for BasicStyle {fn default() -> Self {BasicStyle::Normal}}

/**
 * Foreground color using basic color
 */
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum BasicColor {
    Black        = 30,
    Red          = 31,
    Green        = 32,
    Yellow       = 33,
    Blue         = 34,
    Magenta      = 35,
    Cyan         = 36,
    LightGrey    = 37,
    Default      = 39,
    DarkGrey     = 90,
    LightRed     = 91,
    LightGreen   = 92,
    LightYellow  = 93,
    LightBlue    = 94,
    LightMagenta = 95,
    LightCyan    = 96,
    White        = 97
}

impl Default for BasicColor {fn default() -> Self {BasicColor::Default}}

/**
 * The background of a teerminal using basic color
 */
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum BasicBackground {
    Black        = 40,
    Red          = 41,
    Green        = 42,
    Yellow       = 43,
    Blue         = 44,
    Magenta      = 45,
    Cyan         = 46,
    LightGrey    = 47,
    Default      = 49,
    DarkGrey     = 100,
    LightRed     = 101,
    LightGreen   = 102,
    LightYellow  = 103,
    LightBlue    = 104,
    LightMagenta = 105,
    LightCyan    = 106,
    White        = 107
}

impl Default for BasicBackground {fn default() -> Self {BasicBackground::Default}}

/**
 * A chalk with only 16 colors
 */
#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BasicChalk {
	pub fgcolor: BasicColor,
	pub bgcolor: BasicColor,
	pub styles: Vec<BasicStyle>
}

impl BasicChalk {
	/**
	 * Creates a string which does all of the style,
	 * Helper function for the Chalk implementation
	 */
	fn style(self) -> String {
		let mut style_command = String::with_capacity(12);
		for style in self.styles {
			style_command = format!("{}{};", style_command, style as u8);
		}
		style_command
	}
}

impl Chalk for BasicChalk {

	fn string(self, string: &dyn ToString) -> String {
		format!("\x1b[{};{};{}m{}\x1b[m",
		self.fgcolor.clone() as u8,
		self.bgcolor.clone() as u8,
		self.style(), string.to_string())
	}
}