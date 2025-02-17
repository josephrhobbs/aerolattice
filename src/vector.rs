//! N-dimensional vector implementation.
//!
//! Unlike the `Vector3D` structure, which is used for vectors only in three dimensions,
//! the `Vector` structure can represent vectors of arbitrary dimension.  Use of this
//! structure is recommended when representing linear systems of equations.

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
};

use std::ops::{
    Add,
    Index,
    IndexMut,
    Sub,
};

#[pyclass]
#[derive(Clone, Debug)]
/// A vector in N-dimensional space.
pub struct Vector {
    #[pyo3(get)]
    /// This vector's values.
    pub values: Vec<f64>,

    /// This vector's dimension.
    pub n: usize,
}

#[pymethods]
impl Vector {
    #[new]
    /// Construct a new N-dimensional vector.
    pub fn new(values: Vec<f64>) -> Self {
        Self {
            n: values.len(),
            values,
        }
    }

    /// Scale this vector by a given scalar.
    pub fn scale(&self, scalar: f64) -> Self {
        let mut output = Vec::new();

        for v in &self.values {
            output.push(v * scalar);
        }

        Self {
            n: self.values.len(),
            values: output,
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
    
    #[pyo3(name = "__add__")]
    /// Add two vectors.
    pub fn add(&self, vector: Vector) -> PyResult<Vector> {
        if self.n == vector.n {
            Ok (self.clone() + vector)
        } else {
            Err (PyTypeError::new_err("vectors must have the same dimension"))
        }
    }

    #[pyo3(name = "__sub__")]
    /// Subtract two vectors.
    pub fn sub(&self, vector: Vector) -> PyResult<Vector> {
        if self.n == vector.n {
            Ok (self.clone() - vector)
        } else {
            Err (PyTypeError::new_err("vectors must have the same dimension"))
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, vector: Vector) -> Self::Output {
        let mut output = vec![0.0; self.n];

        for i in 0..self.n {
            output[i] = self[i] + vector[i];
        }

        Vector::new(output)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, vector: Vector) -> Self::Output {
        let mut output = vec![0.0; self.n];

        for i in 0..self.n {
            output[i] = self[i] - vector[i];
        }

        Vector::new(output)
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