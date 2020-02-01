
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

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct BasicChalk {
	pub fgcolor: BasicColor,
	pub bgcolor: BasicColor,
	pub styles: Vec<BasicStyle>
}