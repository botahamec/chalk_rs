use crate::{
	enum_default,
	enum_display,
	enum_fmt_impl,
	enum_impls
};
use std::fmt::Binary;
use std::fmt::Octal;
use std::fmt::Display;
use std::fmt::LowerHex;
use std::fmt::UpperHex;

/** A style to be applied to the text */
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Style {
    Default         = 0,
    Bold            = 1,
	Dim             = 2,
	Italic          = 3,
    Underline       = 4,
	Blink           = 5,
	FastBlink       = 6,
    Invert          = 7,
	Hidden          = 8,
	DoubleUnderline = 21
}

enum_impls!(Style);