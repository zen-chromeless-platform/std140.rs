#![allow(non_camel_case_types)]

//! This module contains types that may be used to define Rust struct types that match the GLSL
//! std140 memory layout.
//!
//! Std140 is a standardized memory layout for GLSL shader interface blocks (e.g. uniform blocks).
//! An interface block is a group op typed GLSL variables. For details on the layout rules for
//! std140, please refer to the section 2.12.6.4 "Standard Uniform Block Layout" of the
//! [OpenGL ES 3.0 Specification](https://www.khronos.org/registry/OpenGL/specs/es/3.0/es_spec_3.0.pdf).
//!
//! This module aims to make it easy to construct and manipulate a block of std140 compatible memory
//! as a Rust struct, such that the struct's memory layout will match a GLSL interface block if
//! every block member is pairwise type-compatible with the struct field in the corresponding
//! position. Position here relates to the order in which block members and struct fields are
//! declared, e.g.: the 1st block member must be compatible with the 1st struct field, the 2nd block
//! member must be compatible with the 2nd struct field, etc. The struct itself has to be marked
//! with the [`#[repr_std140]`][repr_std140] attribute: this ensures the rust compiler will
//! correctly order and align the fields.
//!
//! A GLSL struct type is compatible with a field if this field's type is a Rust struct marked with
//! [`#[repr_std140]`][repr_std140] and this struct's fields are pair-wise compatible with the GLSL
//! struct field in the corresponding position.
//!
//! A GLSL array of GLSL type `T` with compatible type `T_c` (as defined above) and length `N` is
//! compatible with a field of type `std140::array<T_c, N>`.
//!
//! # Example
//!
//! Given the following GLSL declaration of an (uniform) interface block:
//!
//! ```glsl
//! struct PointLight {
//!     vec3 position;
//!     float intensity;
//! }
//!
//! layout(std140) uniform Uniforms {
//!     mat4 transform;
//!     vec3 ambient_light_color;
//!     PointLight lights[2];
//! }
//! ```
//!
//! The following will produce a Rust struct instance with a compatible memory layout:
//!
//! ```rust
//! #[std140::repr_std140]
//! struct PointLight {
//!     position: std140::vec::vec3,
//!     intensity: std140::float,
//! }
//!
//! #[std140::repr_std140]
//! struct Uniforms {
//!     transform: std140::mat::mat4x4,
//!     ambient_light_color: std140::vec::vec3,
//!     lights: std140::array::array<PointLight, 2>
//! }
//!
//! let instance = Uniforms {
//!     transform: std140::mat4x4(
//!         std140::vec::vec4(1.0, 0.0, 0.0, 0.0),
//!         std140::vec::vec4(0.0, 1.0, 0.0, 0.0),
//!         std140::vec::vec4(0.0, 0.0, 1.0, 0.0),
//!         std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
//!     ),
//!     ambient_light_color: std140::vec::vec3(0.2, 0.2, 0.2),
//!     lights: std140::array![
//!         PointLight {
//!             position: std140::vec::vec3(10.0, 0.0, 10.0),
//!             intensity: std140::float(0.5)
//!         },
//!         PointLight {
//!             position: std140::vec::vec3(0.0, 10.0, 10.0),
//!             intensity: std140::float(0.8)
//!         },
//!     ]
//! };
//! ```
//!
//! Note that although the field names match the block member names in this example, this is not
//! strictly necessary: only pairwise field-type compatibility is required.
//!
//! [repr_std140]: attr.repr_std140.html

/// Attribute macro that can be applied to a struct to ensure its representation is compatible with
/// the std140 memory layout convention.
///
/// Can only be applied to a struct if all of its fields implement [ReprStd140].
///
/// Any struct marked with this attribute will automatically implement [Std140Struct]
///
/// # Example
///
/// ```rust
/// #[std140::repr_std140]
/// struct PointLight {
///     position: std140::vec::vec3,
///     intensity: std140::float,
/// }
/// ```
pub use std140_macros::repr_std140;

pub mod array;
pub mod mat;
pub mod unbounded_array;
pub mod vec;

/// Marker trait for types that can be used as fields in structs marked with
/// [`#[repr_std140]`][repr_std140].
///
/// [repr_std140]: attr.repr_std140.html
pub unsafe trait ReprStd140 {}

/// Marker trait for types that can be used as the element type for std140 [array][struct@array]s.
pub unsafe trait Std140ArrayElement: ReprStd140 {}

/// Marker trait for struct types that were marked with [`#[repr_std140]`][repr_std140].
///
/// [repr_std140]: attr.repr_std140.html
pub unsafe trait Std140Struct {}

unsafe impl<T> ReprStd140 for T where T: Std140Struct {}
unsafe impl<T> Std140ArrayElement for T where T: Std140Struct {}

/// Initializes a `std140` [array][array::array].
///
/// # Example
///
/// ```
/// let std140_array: std140::array::array<std140::vec::vec2, 2> = std140::array![
///     std140::vec::vec2(1.0, 0.0),
///     std140::vec::vec2(0.0, 1.0),
/// ];
/// ```
#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => {
        $crate::array::array::new([$crate::array::AlignmentedElement($elem); $n])
    };
    ($($x:expr),*) => {
        $crate::array::array::new([$($crate::array::AlignmentedElement($x) ),*])
    };
    ($($x:expr,)*) => ($crate::array![$($x),*])
}

/// Initializes a `std140` [unbounded_array][unbounded_array::unbounded_array].
#[macro_export]
macro_rules! unbounded_array {
    ($t:ty, $elem:expr; $n:expr) => {
        $crate::unbounded_array::unbounded_array::<$t>::new([$crate::array::AlignmentedElement($elem); $n].to_vec())
    };
    ($t:ty, $($x:expr),*) => {
        $crate::unbounded_array::unbounded_array::<$t>::new([$($crate::array::AlignmentedElement($x) ),*].to_vec())
    };
    ($t:ty, $($x:expr,)*) => ($crate::unbounded_array![$t, $($x),*])
}

#[cfg(test)]
mod tests {
    use super::{array::AlignmentedElement, uint};

    #[test]
    fn it_works() {
        let a = unbounded_array![uint, uint(0)];
        assert_eq!(AlignmentedElement(uint(0)), a[0]);
    }

    #[test]
    fn it_works2() {
        let a = unbounded_array![uint, uint(0);1];
        assert_eq!(AlignmentedElement(uint(0)), a[0]);
    }
}

/// A 32-bit floating point value.
///
/// # Example
///
/// ```
/// let value = std140::float(0.5);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct float(pub f32);

unsafe impl ReprStd140 for float {}
unsafe impl Std140ArrayElement for float {}

/// A 32-bit signed integer value.
///
/// # Example
///
/// ```
/// let value = std140::int(1);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct int(pub i32);

unsafe impl ReprStd140 for int {}
unsafe impl Std140ArrayElement for int {}

/// A 32-bit unsigned integer value.
///
/// # Example
///
/// ```
/// let value = std140::uint(1);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uint(pub u32);

unsafe impl ReprStd140 for uint {}
unsafe impl Std140ArrayElement for uint {}

/// A 32-bit boolean value.
///
/// [boolean::False] is stored identically to a [uint] of `0`; [boolean::True] is stored identically
/// to a [uint] of `1`.
///
/// # Example
///
/// ```
/// use ::std140::{boolean, uint};
///
/// assert_eq!(boolean::from(uint(1)), boolean::True);
/// assert_eq!(boolean::from(uint(0)), boolean::False);
/// ```
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum boolean {
    True = 1,
    False = 0,
}

unsafe impl ReprStd140 for boolean {}
unsafe impl Std140ArrayElement for boolean {}

macro_rules! impl_from_for_boolean {
    ($name:ty, $zero:literal) => {
        impl From<$name> for boolean {
            fn from(value: $name) -> Self {
                if value.0 != $zero {
                    boolean::True
                } else {
                    boolean::False
                }
            }
        }
    };
}

impl_from_for_boolean!(float, 0.);
impl_from_for_boolean!(int, 0);
impl_from_for_boolean!(uint, 0);

impl From<bool> for boolean {
    fn from(value: bool) -> Self {
        if value {
            boolean::True
        } else {
            boolean::False
        }
    }
}

/// Initializes a [mat2x2][crate::mat::mat2x2].
///
/// # Example
///
/// ```
/// let value = std140::mat2x2(
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
/// );
/// ```
pub const fn mat2x2(c0: vec::vec2, c1: vec::vec2) -> mat::mat2x2 {
    mat::mat2x2 {
        columns: array![c0, c1],
    }
}

/// Initializes a [mat2x3][crate::mat::mat2x3].
///
/// # Example
///
/// ```
/// let value = std140::mat2x3(
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat2x3(c0: vec::vec3, c1: vec::vec3) -> mat::mat2x3 {
    mat::mat2x3 {
        columns: array![c0, c1],
    }
}

/// Initializes a [mat2x4][crate::mat::mat2x4].
///
/// # Example
///
/// ```
/// let value = std140::mat2x4(
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat2x4(c0: vec::vec4, c1: vec::vec4) -> mat::mat2x4 {
    mat::mat2x4 {
        columns: array![c0, c1],
    }
}

/// Initializes a [mat3x2][crate::mat::mat3x2].
///
/// # Example
///
/// ```
/// let value = std140::mat3x2(
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
/// );
/// ```
pub const fn mat3x2(c0: vec::vec2, c1: vec::vec2, c2: vec::vec2) -> mat::mat3x2 {
    mat::mat3x2 {
        columns: array![c0, c1, c2],
    }
}

/// Initializes a [mat3x3][crate::mat::mat3x3].
///
/// # Example
///
/// ```
/// let value = std140::mat3x3(
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat3x3(c0: vec::vec3, c1: vec::vec3, c2: vec::vec3) -> mat::mat3x3 {
    mat::mat3x3 {
        columns: array![c0, c1, c2],
    }
}

/// Initializes a [mat3x4][crate::mat::mat3x4].
///
/// # Example
///
/// ```
/// let value = std140::mat3x4(
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat3x4(c0: vec::vec4, c1: vec::vec4, c2: vec::vec4) -> mat::mat3x4 {
    mat::mat3x4 {
        columns: array![c0, c1, c2],
    }
}

/// Initializes a [mat4x2][crate::mat::mat4x2].
///
/// # Example
///
/// ```
/// let value = std140::mat4x2(
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
///     std140::vec::vec2(0.0, 1.0),
/// );
/// ```
pub const fn mat4x2(c0: vec::vec2, c1: vec::vec2, c2: vec::vec2, c3: vec::vec2) -> mat::mat4x2 {
    mat::mat4x2 {
        columns: array![c0, c1, c2, c3],
    }
}

/// Initializes a [mat4x3][crate::mat::mat4x3].
///
/// # Example
///
/// ```
/// let value = std140::mat4x3(
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
///     std140::vec::vec3(0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat4x3(c0: vec::vec3, c1: vec::vec3, c2: vec::vec3, c3: vec::vec3) -> mat::mat4x3 {
    mat::mat4x3 {
        columns: array![c0, c1, c2, c3],
    }
}

/// Initializes a [mat4x4][crate::mat::mat4x4].
///
/// # Example
///
/// ```
/// let value = std140::mat4x4(
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
pub const fn mat4x4(c0: vec::vec4, c1: vec::vec4, c2: vec::vec4, c3: vec::vec4) -> mat::mat4x4 {
    mat::mat4x4 {
        columns: array![c0, c1, c2, c3],
    }
}
