use ::std::{
    ops::{Index,IndexMut},
};

use crate::{
    ReprStd140,
    Std140ArrayElement,
    boolean,
};

/// A column vector of 2 [float][crate::float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::vec2(0.0, 1.0);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec2(pub f32, pub f32);

impl vec2 {
    /// Creates a new `vec2` with zeros in all positions.
    pub const fn zero() -> Self {
        vec2(0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec2 {}
unsafe impl Std140ArrayElement for vec2 {}

impl Index<usize> for vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [float][crate::float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::vec3(0.0, 0.0, 1.0);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec3(pub f32, pub f32, pub f32);

impl vec3 {
    /// Creates a new `vec3` with zeros in all positions.
    pub const fn zero() -> Self {
        vec3(0.0, 0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec3 {}
unsafe impl Std140ArrayElement for vec3 {}

impl Index<usize> for vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [float][crate::float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::vec4(0.0, 0.0, 0.0, 1.0);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec4(pub f32, pub f32, pub f32, pub f32);

impl vec4 {
    /// Creates a new `vec4` with zeros in all positions.
    pub const fn zero() -> Self {
        vec4(0.0, 0.0, 0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec4 {}
unsafe impl Std140ArrayElement for vec4 {}

impl Index<usize> for vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 2 [int][crate::int] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::ivec2(0, 1);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec2(pub i32, pub i32);

impl ivec2 {
    /// Creates a new `ivec2` with zeros in all positions.
    pub const fn zero() -> Self {
        ivec2(0, 0)
    }
}

unsafe impl ReprStd140 for ivec2 {}
unsafe impl Std140ArrayElement for ivec2 {}

impl Index<usize> for ivec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [int][crate::int] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::ivec3(0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec3(pub i32, pub i32, pub i32);

impl ivec3 {
    /// Creates a new `ivec3` with zeros in all positions.
    pub const fn zero() -> Self {
        ivec3(0, 0, 0)
    }
}

unsafe impl ReprStd140 for ivec3 {}
unsafe impl Std140ArrayElement for ivec3 {}

impl Index<usize> for ivec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [int][crate::int] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::ivec4(0, 0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec4(pub i32, pub i32, pub i32, pub i32);

impl ivec4 {
    /// Creates a new `ivec4` with zeros in all positions.
    pub const fn zero() -> Self {
        ivec4(0, 0, 0, 0)
    }
}

unsafe impl ReprStd140 for ivec4 {}
unsafe impl Std140ArrayElement for ivec4 {}

impl Index<usize> for ivec4 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 2 [uint][crate::uint] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::uvec2(0, 1);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec2(pub u32, pub u32);

impl uvec2 {
    /// Creates a new `uvec2` with zeros in all positions.
    pub const fn zero() -> Self {
        uvec2(0, 0)
    }
}

unsafe impl ReprStd140 for uvec2 {}
unsafe impl Std140ArrayElement for uvec2 {}

impl Index<usize> for uvec2 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [uint][crate::uint] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::uvec3(0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec3(pub u32, pub u32, pub u32);

impl uvec3 {
    /// Creates a new `uvec3` with zeros in all positions.
    pub const fn zero() -> Self {
        uvec3(0, 0, 0)
    }
}

unsafe impl ReprStd140 for uvec3 {}
unsafe impl Std140ArrayElement for uvec3 {}

impl Index<usize> for uvec3 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [uint][crate::uint] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::uvec4(0, 0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec4(pub u32, pub u32, pub u32, pub u32);

impl uvec4 {
    /// Creates a new `uvec4` with zeros in all positions.
    pub const fn zero() -> Self {
        uvec4(0, 0, 0, 0)
    }
}

unsafe impl ReprStd140 for uvec4 {}
unsafe impl Std140ArrayElement for uvec4 {}

impl Index<usize> for uvec4 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 2 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::bvec2(std140::boolean::False, std140::boolean::True);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec2(pub boolean, pub boolean);

impl bvec2 {
    /// Creates a new `bvec2` with zeros in all positions.
    pub const fn zero() -> Self {
        Self(boolean::False, boolean::False)
    }
}

unsafe impl ReprStd140 for bvec2 {}
unsafe impl Std140ArrayElement for bvec2 {}

impl Index<usize> for bvec2 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::bvec3(std140::boolean::False, std140::boolean::False, std140::boolean::True);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec3(pub boolean, pub boolean, pub boolean);

impl bvec3 {
    /// Creates a new `bvec3` with zeros in all positions.
    pub const fn zero() -> Self {
        Self(boolean::False, boolean::False, boolean::False)
    }
}

unsafe impl ReprStd140 for bvec3 {}
unsafe impl Std140ArrayElement for bvec3 {}

impl Index<usize> for bvec3 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::vec::bvec4(
///     std140::boolean::False,
///     std140::boolean::False,
///     std140::boolean::False,
///     std140::boolean::True
/// );
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec4(pub boolean, pub boolean, pub boolean, pub boolean);

impl bvec4 {
    /// Creates a new `bvec4` with zeros in all positions.
    pub const fn zero() -> Self {
        Self(boolean::False, boolean::False, boolean::False, boolean::False)
    }
}

unsafe impl ReprStd140 for bvec4 {}
unsafe impl Std140ArrayElement for bvec4 {}

impl Index<usize> for bvec4 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}
