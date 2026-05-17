use std::fmt::Debug;

/// A marker for UTF-16 code unit types.
pub trait CodeUnit: Into<u16> + Debug + Copy + Ord {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BmpChar {
	/// must be in the range of integers from 0 to 0xD7FF or from 0xE000 to 0xFFFF.
	code: u16,
}

impl BmpChar {
	pub const fn new(code: u16) -> Option<Self> {
		match DecodedCodeUnit::from_char_code(code) {
			DecodedCodeUnit::Bmp(v) => Some(v),
			_ => None,
		}
	}

	/// # Safety
	/// The argument must be in the range of integers from 0 to 0xD7FF or from 0xE000 to 0xFFFF.
	pub const unsafe fn new_unchecked(code: u16) -> Self {
		match Self::new(code) {
			Some(v) => v,
			None => unreachable!(),
		}
	}
}

impl From<BmpChar> for u16 {
	fn from(value: BmpChar) -> Self {
		value.code
	}
}

impl CodeUnit for BmpChar {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeadingSurrogateChar {
	/// must be in the range of integers from 0xD800 to 0xDBFF.
	code: u16,
}

impl LeadingSurrogateChar {
	pub const fn new(code: u16) -> Option<Self> {
		match DecodedCodeUnit::from_char_code(code) {
			DecodedCodeUnit::LeadingSurrogate(v) => Some(v),
			_ => None,
		}
	}

	/// # Safety
	/// The argument must be in the range of integers from 0xD800 to 0xDBFF.
	pub const unsafe fn new_unchecked(code: u16) -> Self {
		match Self::new(code) {
			Some(v) => v,
			None => unreachable!(),
		}
	}
}

impl From<LeadingSurrogateChar> for u16 {
	fn from(value: LeadingSurrogateChar) -> Self {
		value.code
	}
}

impl CodeUnit for LeadingSurrogateChar {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrailingSurrogateChar {
	/// must be in the range of integers from 0xDC00 to 0xDFFF.
	code: u16,
}

impl TrailingSurrogateChar {
	pub const fn new(code: u16) -> Option<Self> {
		match DecodedCodeUnit::from_char_code(code) {
			DecodedCodeUnit::TrailingSurrogate(v) => Some(v),
			_ => None,
		}
	}

	/// # Safety
	/// The argument must be in the range of integers from 0xDC00 to 0xDFFF.
	pub const unsafe fn new_unchecked(code: u16) -> Self {
		match Self::new(code) {
			Some(v) => v,
			None => unreachable!(),
		}
	}
}

impl From<TrailingSurrogateChar> for u16 {
	fn from(value: TrailingSurrogateChar) -> Self {
		value.code
	}
}

impl CodeUnit for TrailingSurrogateChar {}

pub enum DecodedCodeUnit {
	Bmp(BmpChar),
	LeadingSurrogate(LeadingSurrogateChar),
	TrailingSurrogate(TrailingSurrogateChar),
}

impl DecodedCodeUnit {
	const fn from_char_code(code: u16) -> Self {
		match code {
			0..=0xD7FF | 0xE000..=0xFFFF => Self::Bmp(BmpChar { code }),
			0xD800..=0xDBFF => Self::LeadingSurrogate(LeadingSurrogateChar { code }),
			0xDC00..=0xDFFF => Self::TrailingSurrogate(TrailingSurrogateChar { code }),
		}
	}
}

impl From<u16> for DecodedCodeUnit {
	fn from(code: u16) -> Self {
		Self::from_char_code(code)
	}
}
