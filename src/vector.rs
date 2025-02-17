//! N-dimensional vector implementation.
//!
//! Unlike the `Vector3D` structure, which is used for vectors only in three dimensions,
//! the `Vector` structure can represent vectors of arbitrary dimension.  Use of this
//! structure is recommended when representing linear systems of equations.

use pyo3::prelude::*;

use std::ops::{
    Index,
    IndexMut,
};

#[pyclass]
#[derive(Clone, Debug)]
/// A vector in N-dimensional space.
pub struct Vector {
    /// This vector's values.
    pub values: Vec<f64>,
}

#[pymethods]
impl Vector {
    #[new]
    /// Construct a new N-dimensional vector.
    pub fn new(values: Vec<f64>) -> Self {
        Self {
            values,
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this vector.
    pub fn py_repr(&self) -> String {
        format!(
            "Vector([{}])",
            self.values.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[pyo3(name = "__str__")]
    /// Display a human-readable representation of this vector.
    pub fn to_string(&self) -> String {
        format!(
            "[{}]",
            self.values.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[pyo3(name = "__getitem__")]
    /// Index into this vector, returning a floating-point value.
    pub fn index(&self, i: usize) -> f64 {
        self[i]
    }

    #[pyo3(name = "__setitem__")]
    /// Index into this vector, setting a floating-point value.
    pub fn index_mut(&mut self, i: usize, value: f64) {
        self[i] = value;
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.values[i]
    }
}