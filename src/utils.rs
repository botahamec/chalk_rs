
/**
 * Implements Default for an enum.
 * Requires the enum to have a variant named "Default"
 */
#[macro_export]
macro_rules! enum_default {
	($name: ident) => {
		impl Default for $name {fn default() -> Self {$name::Default}}
	}
}

/**
 * Implements a fmt trait for an enum.
 * The output is the enum as a number
 */
#[macro_export]
macro_rules! enum_fmt_impl {
	($name: ident, $trait: ident) => {
		impl $trait for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		        $trait::fmt(&(self.clone() as u8), f)
		    }
		}
	}
}

/**
 * Implements the Display trait for an enum.
 * The output is the enum as a number
 */
#[macro_export]
macro_rules! enum_display {
	($name: ident) => {
		impl Display for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		        write!(f, "{}", self.clone() as u8)
		    }
		}
	}
}

/** Implements several traits for a macro */
#[macro_export]
macro_rules! enum_impls {
	($name: ident) => {
		enum_default!($name);
		enum_display!($name);
		enum_fmt_impl!($name, Binary);
		enum_fmt_impl!($name, Octal);
		enum_fmt_impl!($name, LowerHex);
		enum_fmt_impl!($name, UpperHex);
	};
}