use std::{fmt::Display, iter::Peekable};

use crate::string::code_unit::{CodeUnit, DecodedCodeUnit, LeadingSurrogateChar, TrailingSurrogateChar};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodePoint {
	/// must be in the range of integers from 0 to 0x10FFFF.
	cp: u32,
}

impl CodePoint {
	pub const fn from_u32(cp: u32) -> Option<Self> {
		match cp {
			0..=0x10FFFF => Some(Self { cp }),
			_ => None,
		}
	}

	/// # SAFETY
	/// The argument must be in the range of integers from 0 to 0x10FFFF.
	pub const unsafe fn from_u32_unchecked(cp: u32) -> Self {
		match Self::from_u32(cp) {
			Some(v) => v,
			None => unreachable!(),
		}
	}

	pub const fn from_u16(cp: u16) -> Self {
		Self { cp: cp as u32 }
	}

	pub const fn from_char(cp: char) -> Self {
		Self { cp: cp as u32 }
	}

	pub fn from_surrogate_pair(lead: LeadingSurrogateChar, trail: TrailingSurrogateChar) -> Self {
		let lead = u32::from(u16::from(lead));
		let trail = u32::from(u16::from(trail));
		let cp = (lead - 0xD800) * 0x400 + (trail - 0xDC00) + 0x10000;
		Self { cp }
	}

	/// The return value is in the range of integers from 0 to 0x10FFFF.
	pub fn code_point(&self) -> u32 {
		self.cp
	}

	pub fn code_unit_count(&self) -> usize {
		if self.cp < 0x10000 {
			1
		} else {
			2
		}
	}

	pub fn is_unpaired_surrogate(&self) -> bool {
		matches!(self.cp, 0xD800..=0xDFFF)
	}

	pub fn as_char(&self) -> Option<char> {
		char::from_u32(self.code_point())
	}
}

impl From<u16> for CodePoint {
	fn from(cp: u16) -> Self {
		Self::from_u16(cp)
	}
}

impl<T> From<T> for CodePoint where T: CodeUnit {
	fn from(value: T) -> Self {
		Self { cp: Into::<u16>::into(value).into() }
	}
}

impl From<char> for CodePoint {
	fn from(cp: char) -> Self {
		Self::from_char(cp)
	}
}

impl Display for CodePoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.as_char() {
			Some(c) => c.fmt(f),
			None => write!(f, "\\u{}", self.code_point()),
		}
	}
}

pub struct DecodeUtf16<I> where I: Iterator<Item = u16> {
	iter: Peekable<I>,
}

impl<I> Iterator for DecodeUtf16<I> where I: Iterator<Item = u16> {
	type Item = CodePoint;

	fn next(&mut self) -> Option<Self::Item> {
		match DecodedCodeUnit::from(self.iter.next()?) {
			DecodedCodeUnit::Bmp(c) => Some(CodePoint::from(c)),
			DecodedCodeUnit::LeadingSurrogate(lead) => {
				let Some(DecodedCodeUnit::TrailingSurrogate(trail)) = self.iter.peek().map(|&c| c.into()) else {
					return Some(CodePoint::from(lead));
				};
				self.iter.next();
				return Some(CodePoint::from_surrogate_pair(lead, trail));
			},
			DecodedCodeUnit::TrailingSurrogate(c) => Some(CodePoint::from(c)),
		}
	}
}

pub fn decode_utf16<I>(iter: impl IntoIterator<IntoIter = I>) -> DecodeUtf16<I> where I: Iterator<Item = u16> {
	DecodeUtf16 { iter: iter.into_iter().peekable() }
}

#[cfg(test)]
mod tests {
	use super::*;

	mod test_decode_utf16 {
		use super::*;

		#[test]
		fn input_bmp_only() {
			let utf16_seq: Vec<_> = "Hello".encode_utf16().collect();
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_char('H'),
				CodePoint::from_char('e'),
				CodePoint::from_char('l'),
				CodePoint::from_char('l'),
				CodePoint::from_char('o'),
			];
			assert_eq!(actual, expected);
		}

		#[test]
		fn input_unpaired_leading_surrogate_in_middle() {
			let utf16_seq: Vec<_> = vec![0xD800, '0' as u16];
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_u16(0xD800),
				CodePoint::from_char('0'),
			];
			assert_eq!(actual, expected);
		}

		#[test]
		fn input_unpaired_leading_surrogate_at_end() {
			let utf16_seq: Vec<_> = vec!['0' as u16, 0xD800];
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_char('0'),
				CodePoint::from_u16(0xD800),
			];
			assert_eq!(actual, expected);
		}

		#[test]
		fn input_unpaired_trailing_surrogate_at_start() {
			let utf16_seq: Vec<_> = vec![0xDC00, '0' as u16];
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_u16(0xDC00),
				CodePoint::from_char('0'),
			];
			assert_eq!(actual, expected);
		}

		#[test]
		fn input_unpaired_trailing_surrogate_in_middle() {
			let utf16_seq: Vec<_> = vec!['0' as u16, 0xDC00];
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_char('0'),
				CodePoint::from_u16(0xDC00),
			];
			assert_eq!(actual, expected);
		}

		#[test]
		fn input_paired_surrogates() {
			let utf16_seq: Vec<_> = vec![0xD800, 0xDC00];
			let actual: Vec<_> = decode_utf16(utf16_seq).collect();
			let expected = vec![
				CodePoint::from_surrogate_pair(
					unsafe { LeadingSurrogateChar::new_unchecked(0xD800) },
					unsafe { TrailingSurrogateChar::new_unchecked(0xDC00) },
				),
			];
			assert_eq!(actual, expected);
		}
	}
}
