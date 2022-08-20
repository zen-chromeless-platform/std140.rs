use ::std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{ReprStd140, Std140ArrayElement};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(16))]
pub struct AlignmentedElement<T>(pub T)
where
    T: Std140ArrayElement;

impl<T> fmt::Debug for AlignmentedElement<T>
where
    T: Std140ArrayElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Debug>::fmt(&self.0, f)
    }
}

/// Represents an std140 compatible array.
///
/// All elements in an std140 array are aligned to at least 16 bytes.
#[derive(Clone, Copy)]
pub struct array<T, const LEN: usize>([AlignmentedElement<T>; LEN])
where
    T: Std140ArrayElement;

impl<T, const LEN: usize> array<T, { LEN }>
where
    T: Std140ArrayElement,
{
    #[inline]
    pub const fn new(inner: [AlignmentedElement<T>; LEN]) -> Self {
        Self(inner)
    }
}

impl<T, const LEN: usize> From<[AlignmentedElement<T>; LEN]> for array<T, { LEN }>
where
    T: Std140ArrayElement,
{
    #[inline]
    fn from(inner: [AlignmentedElement<T>; LEN]) -> Self {
        Self(inner)
    }
}

impl<T, const LEN: usize> PartialEq for array<T, { LEN }>
where
    T: Std140ArrayElement + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..LEN {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl<T, const LEN: usize> fmt::Debug for array<T, { LEN }>
where
    T: Std140ArrayElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

// TODO: something like this? (if that ever becomes possible)
//impl<T, const LEN: usize> Unsize<slice<T>> for array<T, {LEN}> {}
//
//pub struct slice<T> where T: Std140ArrayElement {
//    internal: *mut [ArrayElementWrapper<T>]
//}

impl<T, const LEN: usize> Deref for array<T, { LEN }>
where
    T: Std140ArrayElement,
{
    type Target = [AlignmentedElement<T>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T, const LEN: usize> DerefMut for array<T, { LEN }>
where
    T: Std140ArrayElement,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

unsafe impl<T, const LEN: usize> ReprStd140 for array<T, { LEN }> where T: Std140ArrayElement {}
