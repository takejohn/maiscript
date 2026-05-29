use std::{
    fmt::{Debug, Display},
    iter::Peekable,
};

use arrayvec::ArrayVec;

const LEADING_SURROGATE_MIN: u16 = 0xD800;
const LEADING_SURROGATE_MAX: u16 = 0xDBFF;
const TRAILING_SURROGATE_MIN: u16 = 0xDC00;
const TRAILING_SURROGATE_MAX: u16 = 0xDFFF;

const BASIC_PLANE_MIN: u32 = 0;
const SURROGATE_MIN: u32 = LEADING_SURROGATE_MIN as u32;
const SURROGATE_MAX: u32 = TRAILING_SURROGATE_MAX as u32;
const BASIC_PLANE_MAX: u32 = 0xFFFF;
const SUPPLEMENTARY_PLANES_MIN: u32 = 0x10000;
const SUPPLEMENTARY_PLANES_MAX: u32 = 0x10FFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CodeUnitCount {
    ONE = 1,
    TWO = 2,
}

/// A Unicode code point.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodePoint {
    /// must be in the range of integers from 0 to 0x10FFFF.
    value: u32,
}

impl CodePoint {
    pub const fn from_u32(value: u32) -> Option<Self> {
        match value {
            BASIC_PLANE_MIN..=SUPPLEMENTARY_PLANES_MAX => Some(Self { value }),
            _ => None,
        }
    }

    pub const fn from_u16(value: u16) -> Self {
        Self {
            value: value as u32,
        }
    }

    pub const fn from_char(value: char) -> Self {
        Self {
            value: value as u32,
        }
    }

    /// The return value is in the range of integers from 0 to 0x10FFFF.
    pub fn as_u32(self) -> u32 {
        self.value
    }

    pub fn is_unpaired_surrogate(&self) -> bool {
        self.value >= SURROGATE_MIN && self.value <= SURROGATE_MAX
    }

    pub fn as_char(&self) -> Option<char> {
        char::from_u32(self.as_u32())
    }

    pub fn code_unit_count(&self) -> CodeUnitCount {
        match self.value {
            BASIC_PLANE_MIN..=BASIC_PLANE_MAX => CodeUnitCount::ONE,
            SUPPLEMENTARY_PLANES_MIN..=SUPPLEMENTARY_PLANES_MAX => CodeUnitCount::TWO,
            _ => unreachable!(),
        }
    }

    pub fn decode_utf16<I>(iter: impl IntoIterator<IntoIter = I>) -> DecodeUtf16<I>
    where
        I: Iterator<Item = u16>,
    {
        DecodeUtf16 {
            iter: iter.into_iter().peekable(),
        }
    }

    pub fn encode_utf16(self) -> ArrayVec<u16, 2> {
        let value = self.value;
        match self.code_unit_count() {
            CodeUnitCount::ONE => ArrayVec::from_iter(std::iter::once(value as u16)),
            CodeUnitCount::TWO => ArrayVec::from([
                ((value - 0x10000) / 0x400 + 0xD800) as u16,
                ((value - 0x10000) % 0x400 + 0xDC00) as u16,
            ]),
        }
    }
}

impl From<u16> for CodePoint {
    fn from(cp: u16) -> Self {
        Self::from_u16(cp)
    }
}

impl From<char> for CodePoint {
    fn from(cp: char) -> Self {
        Self::from_char(cp)
    }
}

impl Debug for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_char() {
            Some(ch) => Debug::fmt(&ch, f),
            None => write!(f, "'\\u{:04}'", self.as_u32() as u16),
        }
    }
}

impl Display for CodePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_char() {
            Some(c) => Display::fmt(&c, f),
            None => Display::fmt(&char::REPLACEMENT_CHARACTER, f),
        }
    }
}

pub struct DecodeUtf16<I>
where
    I: Iterator<Item = u16>,
{
    iter: Peekable<I>,
}

impl<I> Iterator for DecodeUtf16<I>
where
    I: Iterator<Item = u16>,
{
    type Item = CodePoint;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.iter.next()? {
            lead @ LEADING_SURROGATE_MIN..=LEADING_SURROGATE_MAX => {
                match self
                    .iter
                    .next_if(|&v| v >= TRAILING_SURROGATE_MIN && v <= TRAILING_SURROGATE_MAX)
                {
                    Some(trail) => CodePoint {
                        value: ((lead as u32) - 0xD800) * 0x400
                            + ((trail as u32) - 0xDC00)
                            + 0x10000,
                    },
                    None => CodePoint::from_u16(lead),
                }
            }
            value => CodePoint::from_u16(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_decode_utf16 {
        use super::*;

        #[test]
        fn input_bmp_only() {
            let utf16_seq: Vec<_> = "Hello".encode_utf16().collect();
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
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
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
            let expected = vec![CodePoint::from_u16(0xD800), CodePoint::from_char('0')];
            assert_eq!(actual, expected);
        }

        #[test]
        fn input_unpaired_leading_surrogate_at_end() {
            let utf16_seq: Vec<_> = vec!['0' as u16, 0xD800];
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
            let expected = vec![CodePoint::from_char('0'), CodePoint::from_u16(0xD800)];
            assert_eq!(actual, expected);
        }

        #[test]
        fn input_unpaired_trailing_surrogate_at_start() {
            let utf16_seq: Vec<_> = vec![0xDC00, '0' as u16];
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
            let expected = vec![CodePoint::from_u16(0xDC00), CodePoint::from_char('0')];
            assert_eq!(actual, expected);
        }

        #[test]
        fn input_unpaired_trailing_surrogate_in_middle() {
            let utf16_seq: Vec<_> = vec!['0' as u16, 0xDC00];
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
            let expected = vec![CodePoint::from_char('0'), CodePoint::from_u16(0xDC00)];
            assert_eq!(actual, expected);
        }

        #[test]
        fn input_paired_surrogates() {
            let utf16_seq: Vec<_> = vec![0xD800, 0xDC00];
            let actual: Vec<_> = CodePoint::decode_utf16(utf16_seq).collect();
            let expected = vec![CodePoint::from_char('\u{10000}')];
            assert_eq!(actual, expected);
        }
    }
}
