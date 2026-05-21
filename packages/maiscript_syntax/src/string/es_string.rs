use std::{borrow::Borrow, char, fmt::{Debug, Display, Write}, ops::Deref};

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::CodePoint;

/// ECMAScript String
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EsString(Vec<u16>);

impl EsString {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn from_char_code(char_code: impl IntoIterator<Item = u16>) -> Self {
		Self(Vec::from_iter(char_code.into_iter()))
	}

	pub fn as_es_str(&self) -> &EsStr {
		self
	}

	pub fn push_char_code(&mut self, ch: u16) {
		self.0.push(ch);
	}

	pub fn push_code_point(&mut self, cp: CodePoint) {
		self.0.extend(cp.code_units());
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Deref for EsString {
	type Target = EsStr;

	fn deref(&self) -> &Self::Target {
		let src: &[u16] = self.0.as_slice();
		EsStr::from_u16s(src)
	}
}

impl AsRef<EsStr> for EsString {
	fn as_ref(&self) -> &EsStr {
		self
	}
}

impl Display for EsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<EsStr as Display>::fmt(&self, f)
	}
}

impl From<&EsStr> for EsString {
	fn from(value: &EsStr) -> Self {
		Self::from_char_code(value)
	}
}

impl From<&str> for EsString {
	fn from(value: &str) -> Self {
		Self::from_char_code(value.encode_utf16())
	}
}

impl Borrow<EsStr> for EsString {
	fn borrow(&self) -> &EsStr {
		self
	}
}

#[derive(Debug, RefCastCustom, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EsStr([u16]);

impl EsStr {
	#[ref_cast_custom]
	pub const fn from_u16s(from: &[u16]) -> &Self;
}

impl Display for EsStr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for c in char::decode_utf16(self.0.iter().copied()) {
			f.write_char(c.unwrap_or(char::REPLACEMENT_CHARACTER))?;
		}
		Ok(())
	}
}

impl<'a> IntoIterator for &'a EsStr {
	type Item = u16;

	type IntoIter = std::iter::Copied<std::slice::Iter<'a, u16>>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter().copied()
	}
}

impl ToOwned for EsStr {
	type Owned = EsString;

	fn to_owned(&self) -> Self::Owned {
		EsString::from(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn es_str_is_interpreted_as_utf16() {
		let utf16_seq: Vec<_> = "Hello, world!".encode_utf16().collect();
		let actual = EsString::from_char_code(utf16_seq).to_string();
		assert_eq!(actual, "Hello, world!");
	}

	#[test]
	fn invalid_code_is_replaced() {
		let utf16_seq: Vec<_> = vec![0xd800];
		let actual = EsString::from_char_code(utf16_seq).to_string();
		assert_eq!(actual, char::REPLACEMENT_CHARACTER.to_string());
	}
}
