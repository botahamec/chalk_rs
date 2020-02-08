
use crate::{
	Chalk,
	enum_default,
	enum_display,
	enum_fmt_impl,
	enum_impls,
	chalk_trait_fns,
	set_style,
	add_style,
	fn_alias,
	style::{
		Style,
		ChalkStyle
	}
};

use std::ops::Add;
use std::ops::AddAssign;

/** A chalk with 256 colors */
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct AnsiChalk {
	pub color: u8,
	pub background: u8,
	pub styles: Vec<Style>
}