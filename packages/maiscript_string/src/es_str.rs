use std::fmt::{Display, Write};

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::EsString;

#[derive(Debug, RefCastCustom, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EsStr([u16]);

impl EsStr {
	#[ref_cast_custom]
	pub const fn from_utf16(from: &[u16]) -> &Self;

	pub fn as_u16s(&self) -> &[u16] {
		&self.0
	}
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
