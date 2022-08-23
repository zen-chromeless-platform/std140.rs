use ::std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{array::AlignmentedElement, ReprStd140, Std140ArrayElement};

/// Represents an std140 compatible unbounded array.
///
/// All elements in an std140 array are aligned to at least 16 bytes.
#[derive(Clone)]
#[repr(C, align(16))]
pub struct unbounded_array<T>(Vec<AlignmentedElement<T>>)
where
    T: Std140ArrayElement;

impl<T> unbounded_array<T>
where
    T: Std140ArrayElement,
{
    #[inline]
    pub const fn new(inner: Vec<AlignmentedElement<T>>) -> Self {
        Self(inner)
    }
}

impl<I> FromIterator<AlignmentedElement<I>> for unbounded_array<I>
where
    I: Std140ArrayElement,
{
    fn from_iter<T: IntoIterator<Item = AlignmentedElement<I>>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T> fmt::Debug for unbounded_array<T>
where
    T: Std140ArrayElement + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<T> Deref for unbounded_array<T>
where
    T: Std140ArrayElement,
{
    type Target = Vec<AlignmentedElement<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for unbounded_array<T>
where
    T: Std140ArrayElement,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<T> ReprStd140 for unbounded_array<T> where T: Std140ArrayElement {}

#[cfg(test)]
mod tests {
    use super::unbounded_array;

    #[test]
    fn it_works() {
        assert_eq!(16, std::mem::align_of::<unbounded_array<crate::uint>>());
    }
}
