use std::{char, fmt::{Debug, Display, Write}, mem, ops::Deref};

/// ECMAScript String
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EsString(Vec<u16>);

impl EsString {
	pub fn from_char_code(char_code: impl IntoIterator<Item = u16>) -> Self {
		Self(Vec::from_iter(char_code.into_iter()))
	}

	pub fn as_es_str(&self) -> &EsStr {
		self
	}
}

impl Deref for EsString {
	type Target = EsStr;

	fn deref(&self) -> &Self::Target {
		let src: &[u16] = self.0.as_slice();
		// SAFETY: EsStr is just a wrapper of [u16] with #[repr(transparent)],
		// so transmuting &[u16] to &EsStr is safe.
		let dst: &EsStr = unsafe { mem::transmute(src) };
		dst
	}
}

impl AsRef<EsStr> for EsString {
	fn as_ref(&self) -> &EsStr {
		self
	}
}

impl Display for EsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.as_ref().fmt(f)
	}
}

#[repr(transparent)]
pub struct EsStr([u16]);

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
