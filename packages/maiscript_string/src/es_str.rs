mod slice_index;

use std::{
    fmt::{Display, Write},
    ops::Index,
};

use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::{CodePoint, EsString, es_str::slice_index::EsStrIndex};

#[derive(Debug, RefCastCustom, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct EsStr([u16]);

impl EsStr {
    #[ref_cast_custom]
    pub const fn from_utf16(from: &[u16]) -> &Self;

    pub fn as_u16s(&self) -> &[u16] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get<I>(&self, i: I) -> Option<&I::Output>
    where
        I: EsStrIndex,
    {
        i.get(self)
    }

    pub fn code_point_at(&self, i: usize) -> Option<CodePoint> {
        let u16s = self.get(i..)?.into_iter();
        let mut decoder = CodePoint::decode_utf16(u16s);
        decoder.next()
    }

    pub fn starts_with(&self, pat: &EsStr) -> bool {
        self.0.starts_with(&pat.0)
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

impl<I> Index<I> for EsStr
where
    I: EsStrIndex,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.index(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod code_point {
        use super::*;

        #[test]
        fn bmp_at_start() {
            let s = EsStr::from_utf16(&['A' as u16, 'B' as u16, 'C' as u16]);
            assert_eq!(s.code_point_at(0), Some(CodePoint::from_char('A')));
        }

        #[test]
        fn surrogate_pair_hi() {
            let s = EsStr::from_utf16(&[0xd83d, 0xde0d]);
            assert_eq!(s.code_point_at(0), Some(CodePoint::from_char('\u{1f60d}')));
        }

        #[test]
        fn surrogate_pair_lo() {
            let s = EsStr::from_utf16(&[0xd83d, 0xde0d]);
            assert_eq!(s.code_point_at(1), Some(CodePoint::from_u16(0xde0d)));
        }

        #[test]
        fn end() {
            let s = EsStr::from_utf16(&['A' as u16, 'B' as u16, 'C' as u16]);
            assert_eq!(s.code_point_at(3), None);
        }

        #[test]
        fn out_of_bounds() {
            let s = EsStr::from_utf16(&['A' as u16, 'B' as u16, 'C' as u16]);
            assert_eq!(s.code_point_at(42), None);
        }
    }
}
