//! Three-dimensional vector implementation.
//!
//! Unlike the `Vector` structure, which is used for vectors of arbitrary dimension,
//! the `Vector3D` structure is specifically intended for representing points and
//! directions in three-dimensional Cartesian space.  The `Vector` structure is
//! primarily dedicated to linear algebra computations and is not recommended for
//! use when representing objects or fluid flows.

use pyo3::prelude::*;

use std::ops::{
    Add,
    Mul,
    Sub,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// A vector in three-dimensional space.
pub struct Vector3D {
    #[pyo3(get, set)]
    /// X coordinate.
    pub x: f64,

    #[pyo3(get, set)]
    /// Y coordinate.
    pub y: f64,

    #[pyo3(get, set)]
    /// Z coordinate.
    pub z: f64,
}

#[pymethods]
impl Vector3D {
    #[new]
    /// Construct a new vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this vector.
    pub fn py_repr(&self) -> String {
        format!(
            "Vector3D({}, {}, {})",
            self.x,
            self.y,
            self.z,
        )
    }

    #[pyo3(name = "__str__")]
    /// Display a human-readable representation of this vector.
    pub fn to_string(&self) -> String {
        format!(
            "({}, {}, {})",
            self.x,
            self.y,
            self.z,
        )
    }

    #[pyo3(name = "__add__")]
    /// Add two vectors.
    pub fn add(&self, other: Self) -> Self {
        *self + other
    }

    #[pyo3(name = "__sub__")]
    /// Subtract two vectors.
    pub fn sub(&self, other: Self) -> Self {
        *self - other
    }

    #[pyo3(name = "__matmul__")]
    /// Calculate the dot product of two vectors.
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[pyo3(name = "__mul__")]
    /// Calculate the cross product of two vectors.
    pub fn cross(&self, other: Self) -> Self {
        *self * other
    }

    #[pyo3(name = "__abs__")]
    /// Calculate the norm of this vector.
    pub fn abs(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    /// Return the unit vector parallel to this vector.
    pub fn normalize(&self) -> Self {
        let abs = self.abs();

        Self {
            x: self.x / abs,
            y: self.y / abs,
            z: self.z / abs,
        }
    }

    /// Scale this vector by a scalar.
    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Add<Vector3D> for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}