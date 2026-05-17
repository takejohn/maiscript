use crate::{CodePoint, string::code_unit::{BmpChar, DecodedCodeUnit, LeadingSurrogateChar, TrailingSurrogateChar}};

pub enum Utf16Char {
	Bmp(BmpChar),
	UnpairedLeadingSurrogate(LeadingSurrogateChar),
	UnpairedTrailingSurrogate(TrailingSurrogateChar),
	SurrogatePair(LeadingSurrogateChar, TrailingSurrogateChar),
}

impl From<CodePoint> for Utf16Char {
	fn from(value: CodePoint) -> Self {
		let cp = value.code_point();
		match cp {
			0..=0xFFFF => match DecodedCodeUnit::from(cp as u16) {
				DecodedCodeUnit::Bmp(c) => Utf16Char::Bmp(c),
				DecodedCodeUnit::LeadingSurrogate(c) => Utf16Char::UnpairedLeadingSurrogate(c),
				DecodedCodeUnit::TrailingSurrogate(c) => Utf16Char::UnpairedTrailingSurrogate(c),
			},
			0x10000..=0x10FFFF => {
				// cu1 is in the range of integers from 0xD800 to 0xDBFF
				let cu1 = ((cp - 0x10000) / 0x400 + 0xD800) as u16;
				// cu2 is in the range of integers from 0xDC00 to 0xDFFF
				let cu2 = ((cp - 0x10000) % 0x400 + 0xDC00) as u16;
				Self::SurrogatePair(
					// SAFETY: It is safe because cu1 is in the range of integers from 0xD800 to 0xDBFF
					unsafe { LeadingSurrogateChar::new_unchecked(cu1) },
					// SAFETY: It is safe because cu2 is in the range of integers from 0xDC00 to 0xDFFF
					unsafe { TrailingSurrogateChar::new_unchecked(cu2) },
				)
			},
			0x110000.. => unreachable!(),
		}
	}
}
