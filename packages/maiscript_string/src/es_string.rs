use std::{borrow::Borrow, fmt::{Debug, Display}, ops::Deref};

use crate::{CodePoint, EsStr};

/// ECMAScript String
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct EsString(Vec<u16>);

impl EsString {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn from_utf16(vec: Vec<u16>) -> Self {
		Self(vec)
	}

	pub fn from_code_points(code_points: impl IntoIterator<Item = CodePoint>) -> Self {
		Self(code_points.into_iter().map(|cp| cp.encode_utf16()).flatten().collect())
	}

	pub fn as_es_str(&self) -> &EsStr {
		self
	}

	pub fn reserve(&mut self, additional: usize) {
		self.0.reserve(additional);
	}

	pub fn push_char_code(&mut self, ch: u16) {
		self.0.push(ch);
	}

	pub fn push_code_point(&mut self, cp: CodePoint) {
		self.0.extend_from_slice(&cp.encode_utf16());
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}
}

impl Deref for EsString {
	type Target = EsStr;

	fn deref(&self) -> &Self::Target {
		let src: &[u16] = self.0.as_slice();
		EsStr::from_utf16(src)
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
		Self::from_utf16(value.as_u16s().to_vec())
	}
}

impl From<&str> for EsString {
	fn from(value: &str) -> Self {
		Self::from_utf16(value.encode_utf16().collect())
	}
}

impl Borrow<EsStr> for EsString {
	fn borrow(&self) -> &EsStr {
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn es_str_is_interpreted_as_utf16() {
		let utf16_seq: Vec<_> = "Hello, world!".encode_utf16().collect();
		let actual = EsString::from_utf16(utf16_seq).to_string();
		assert_eq!(actual, "Hello, world!");
	}

	#[test]
	fn invalid_code_is_replaced() {
		let utf16_seq: Vec<_> = vec![0xd800];
		let actual = EsString::from_utf16(utf16_seq).to_string();
		assert_eq!(actual, char::REPLACEMENT_CHARACTER.to_string());
	}
}
