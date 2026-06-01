use std::{
    ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    slice::SliceIndex,
};

use super::EsStr;

trait RangeIndex: SliceIndex<[u16], Output = [u16]> {}
impl RangeIndex for (Bound<usize>, Bound<usize>) {}
impl RangeIndex for Range<usize> {}
impl RangeIndex for RangeFrom<usize> {}
impl RangeIndex for RangeFull {}
impl RangeIndex for RangeInclusive<usize> {}
impl RangeIndex for RangeTo<usize> {}
impl RangeIndex for RangeToInclusive<usize> {}

trait Sealed {}
impl<T> Sealed for T where T: RangeIndex {}
impl Sealed for usize {}

#[allow(private_bounds)]
pub trait EsStrIndex: Sealed {
    type Output: ?Sized;

    fn get(self, slice: &EsStr) -> Option<&Self::Output>;

    fn index(self, slice: &EsStr) -> &Self::Output;
}

impl<T> EsStrIndex for T
where
    T: RangeIndex,
{
    type Output = EsStr;

    fn get(self, slice: &EsStr) -> Option<&Self::Output> {
        slice.as_u16s().get(self).map(EsStr::from_utf16)
    }

    fn index(self, slice: &EsStr) -> &Self::Output {
        EsStr::from_utf16(&slice.as_u16s()[self])
    }
}

impl EsStrIndex for usize {
    type Output = u16;

    fn get(self, slice: &EsStr) -> Option<&Self::Output> {
        slice.as_u16s().get(self)
    }

    fn index(self, slice: &EsStr) -> &Self::Output {
        &slice.as_u16s()[self]
    }
}
