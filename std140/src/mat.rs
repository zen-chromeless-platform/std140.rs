use ::std::{
    fmt,
    ops::{Deref, DerefMut},
};

use crate::{array, vec, ReprStd140, Std140ArrayElement};

/// A matrix with 2 columns and 2 rows, represented by 2 `vec2` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x2 {
    pub(super) columns: array::array<vec::vec2, 2>,
}

impl mat2x2 {
    /// Creates a new `mat2x2` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat2x2(vec::vec2::zero(), vec::vec2::zero())
    }

    pub const fn identity() -> Self {
        crate::mat2x2(
            vec::vec2(1., 0.),
            vec::vec2(0., 1.),
        )
    }    
}

unsafe impl ReprStd140 for mat2x2 {}
unsafe impl Std140ArrayElement for mat2x2 {}

impl Deref for mat2x2 {
    type Target = array::array<vec::vec2, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x2{:?}", &self.columns))
    }
}

/// A matrix with 2 columns and 3 rows, represented by 2 `vec3` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x3 {
    pub(super) columns: array::array<vec::vec3, 2>,
}

impl mat2x3 {
    /// Creates a new `mat2x3` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat2x3(vec::vec3::zero(), vec::vec3::zero())
    }
}

unsafe impl ReprStd140 for mat2x3 {}
unsafe impl Std140ArrayElement for mat2x3 {}

impl Deref for mat2x3 {
    type Target = array::array<vec::vec3, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x3{:?}", &self.columns))
    }
}

/// A matrix with 2 columns and 4 rows, represented by 2 `vec4` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x4 {
    pub(super) columns: array::array<vec::vec4, 2>,
}

impl mat2x4 {
    /// Creates a new `mat2x4` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat2x4(vec::vec4::zero(), vec::vec4::zero())
    }
}

unsafe impl ReprStd140 for mat2x4 {}
unsafe impl Std140ArrayElement for mat2x4 {}

impl Deref for mat2x4 {
    type Target = array::array<vec::vec4, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x4{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 2 rows, represented by 3 `vec2` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x2 {
    pub(super) columns: array::array<vec::vec2, 3>,
}

impl mat3x2 {
    /// Creates a new `mat3x2` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat3x2(vec::vec2::zero(), vec::vec2::zero(), vec::vec2::zero())
    }
}

unsafe impl ReprStd140 for mat3x2 {}
unsafe impl Std140ArrayElement for mat3x2 {}

impl Deref for mat3x2 {
    type Target = array::array<vec::vec2, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x2{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 3 rows, represented by 3 `vec3` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x3 {
    pub(super) columns: array::array<vec::vec3, 3>,
}

impl mat3x3 {
    /// Creates a new `mat3x3` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat3x3(vec::vec3::zero(), vec::vec3::zero(), vec::vec3::zero())
    }

    pub const fn identity() -> Self {
        crate::mat3x3(
            vec::vec3(1., 0., 0.),
            vec::vec3(0., 1., 0.),
            vec::vec3(0., 0., 1.),
        )
    }    
}

unsafe impl ReprStd140 for mat3x3 {}
unsafe impl Std140ArrayElement for mat3x3 {}

impl Deref for mat3x3 {
    type Target = array::array<vec::vec3, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x3{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 4 rows, represented by 3 `vec4` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x4 {
    pub(super) columns: array::array<vec::vec4, 3>,
}

impl mat3x4 {
    /// Creates a new `mat3x4` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat3x4(vec::vec4::zero(), vec::vec4::zero(), vec::vec4::zero())
    }
}

unsafe impl ReprStd140 for mat3x4 {}
unsafe impl Std140ArrayElement for mat3x4 {}

impl Deref for mat3x4 {
    type Target = array::array<vec::vec4, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x4{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 2 rows, represented by 4 `vec2` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x2 {
    pub(super) columns: array::array<vec::vec2, 4>,
}

impl mat4x2 {
    /// Creates a new `mat4x2` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat4x2(vec::vec2::zero(), vec::vec2::zero(), vec::vec2::zero(), vec::vec2::zero())
    }
}

unsafe impl ReprStd140 for mat4x2 {}
unsafe impl Std140ArrayElement for mat4x2 {}

impl Deref for mat4x2 {
    type Target = array::array<vec::vec2, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x2{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 3 rows, represented by 4 `vec3` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x3 {
    pub(super) columns: array::array<vec::vec3, 4>,
}

impl mat4x3 {
    /// Creates a new `mat4x3` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat4x3(vec::vec3::zero(), vec::vec3::zero(), vec::vec3::zero(), vec::vec3::zero())
    }
}

unsafe impl ReprStd140 for mat4x3 {}
unsafe impl Std140ArrayElement for mat4x3 {}

impl Deref for mat4x3 {
    type Target = array::array<vec::vec3, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x3{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 4 rows, represented by 4 `vec4` vectors.
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x4 {
    pub(super) columns: array::array<vec::vec4, 4>,
}

impl mat4x4 {
    /// Creates a new `mat4x4` with zeros in all positions.
    pub const fn zero() -> Self {
        crate::mat4x4(vec::vec4::zero(), vec::vec4::zero(), vec::vec4::zero(), vec::vec4::zero())
    }

    pub const fn identity() -> Self {
        crate::mat4x4(
            vec::vec4(1., 0., 0., 0.),
            vec::vec4(0., 1., 0., 0.),
            vec::vec4(0., 0., 1., 0.),
            vec::vec4(0., 0., 0., 1.),
        )
    }    
}

unsafe impl ReprStd140 for mat4x4 {}
unsafe impl Std140ArrayElement for mat4x4 {}

impl Deref for mat4x4 {
    type Target = array::array<vec::vec4, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x4{:?}", &self.columns))
    }
}
