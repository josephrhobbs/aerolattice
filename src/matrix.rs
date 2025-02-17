//! N-dimensional square matrix implementation.
//!
//! This data structure is designed to work in tandem with the `Vector`
//! implementation for solving linear systems of equations.  It is not
//! intended for operations in three-dimensional Cartesian space in the
//! same manner that the `Vector3D` structure is.

use std::ops::{
    Add,
    Index,
    IndexMut,
    Mul,
    Sub,
};

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
};

use crate::Vector;

#[pyclass]
#[derive(Clone, Debug)]
/// A matrix in N-dimensional space.
pub struct Matrix {
    /// This matrix's values.
    pub values: Vec<Vec<f64>>,

    /// This matrix's dimension.
    pub n: usize,
}

#[pymethods]
impl Matrix {
    #[new]
    /// Construct a new N-dimensional vector.
    pub fn py_new(values: Vec<Vec<f64>>) -> PyResult<Self> {
        let row_lens = values.iter().map(|r| r.len()).collect::<Vec<usize>>();

        // Make sure all rows are the same length
        for value in &row_lens {
            if *value != row_lens[0] {
                return Err (PyTypeError::new_err("matrix must be square"));
            }
        }

        // Make sure column count equals row count
        if row_lens.len() != row_lens[0] {
            return Err (PyTypeError::new_err("matrix must be square"));
        }

        Ok (Self::new(values))
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this matrix.
    pub fn py_repr(&self) -> String {
        format!(
            "Matrix([{}])",
            self.values.iter()
                .map(|r| format!("[{}]", r.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[pyo3(name = "__str__")]
    /// Display a human-readable representation of this matrix.
    pub fn to_string(&self) -> String {
        format!(
            "[{}]",
            self.values.iter()
                .map(|r| format!("[{}]", r.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ")))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }

    #[pyo3(name = "__getitem__")]
    /// Index into this matrix, returning a floating-point value.
    pub fn index(&self, i: (usize, usize)) -> f64 {
        self[i]
    }

    #[pyo3(name = "__setitem__")]
    /// Index into this matrix, setting a floating-point value.
    pub fn index_mut(&mut self, i: (usize, usize), value: f64) {
        self[i] = value;
    }

    #[pyo3(name = "__add__")]
    /// Add two matrices.
    pub fn add(&self, matrix: Matrix) -> PyResult<Matrix> {
        if self.n == matrix.n {
            Ok (self.clone() + matrix)
        } else {
            Err (PyTypeError::new_err("matrices must have the same dimension"))
        }
    }

    #[pyo3(name = "__sub__")]
    /// Subtract two matrices.
    pub fn sub(&self, matrix: Matrix) -> PyResult<Matrix> {
        if self.n == matrix.n {
            Ok (self.clone() - matrix)
        } else {
            Err (PyTypeError::new_err("matrices must have the same dimension"))
        }
    }

    #[pyo3(name = "__matmul__")]
    /// Multiply this matrix by a matrix, returning a resultant matrix.
    pub fn times_matrix(&self, matrix: Matrix) -> PyResult<Matrix> {
        if self.n == matrix.n {
            Ok (self.clone() * matrix)
        } else {
            Err (PyTypeError::new_err("matrices must have the same dimension"))
        }
    }

    #[pyo3(name = "__mul__")]
    /// Multiply this matrix by a vector, returning a resultant vector.
    pub fn times_vector(&self, vector: Vector) -> PyResult<Vector> {
        if self.n == vector.n {
            Ok (self.clone() * vector)
        } else {
            Err (PyTypeError::new_err("matrix and vector must have the same dimension"))
        }
    }

    /// Compute the inverse of this matrix.
    pub fn inverse(&self) -> Self {
        let mut output = self.clone();
        let mut inverse = self.identity();

        for i in 0..self.n {
            // Determine the index of the row with the largest pivot
            // Start from the working row
            let mut j = i;
            for k in i..self.n {
                if output[(k, i)] > output[(i, i)] {
                    j = k;
                }
            }

            // Swap largest pivot to working row
            output.swaprow(i, j);
            inverse.swaprow(i, j);

            // Normalize this row
            let s = 1.0 / output[(i, i)];
            output.scalerow(i, s);
            inverse.scalerow(i, s);

            // Subtract this row from all lower rows
            for k in (i + 1)..self.n {
                let s = output[(k, i)];
                output.subrow(k, i, s);
                inverse.subrow(k, i, s);
            }
        }

        // We're now in upper triangular form, let's get to GJ normal form

        for i in 0..self.n {
            for j in (i + 1)..self.n {
                let s = output[(i, j)];
                output.subrow(i, j, s);
                inverse.subrow(i, j, s);
            }
        }

        inverse
    }
}

impl Matrix {
    /// Construct a new matrix.
    pub fn new(values: Vec<Vec<f64>>) -> Self {
        Self {
            n: values.len(),
            values,
        }
    }

    /// Construct an identity matrix of the same size as this one.
    fn identity(&self) -> Self {
        let mut output = self.clone();

        for i in 0..self.n {
            for j in 0..self.n {
                output[(i, j)] = if i == j {
                    1.0
                } else {
                    0.0
                };
            }
        }

        output
    }

    /// Swap rows `i` and `j`.
    fn swaprow(&mut self, i: usize, j: usize) {
        let temp = self.values[i].clone();
        self.values[i] = self.values[j].clone();
        self.values[j] = temp;
    }

    /// Scale row `i` by factor `s`.
    fn scalerow(&mut self, i: usize, s: f64) {
        for j in 0..self.values.len() {
            self[(i, j)] *= s;
        }
    }

    /// Subtract `s` times row `j` from row `i`.
    fn subrow(&mut self, i: usize, j: usize, s: f64) {
        for k in 0..self.values.len() {
            self[(i, k)] -= s * self[(j, k)];
        }
    }
}


impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, matrix: Matrix) -> Self::Output {
        let mut output = Vec::new();

        for i in 0..self.n {
            let mut row = vec![0.0; self.n];

            for j in 0..self.n {
                row[j] = self[(i, j)] + matrix[(i, j)];
            }

            output.push(row);
        }

        Matrix::new(output)
    }
}

impl Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, matrix: Matrix) -> Self::Output {
        let mut output = Vec::new();

        for i in 0..self.n {
            let mut row = vec![0.0; self.n];

            for j in 0..self.n {
                row[j] = self[(i, j)] - matrix[(i, j)];
            }

            output.push(row);
        }

        Matrix::new(output)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Self::Output {
        let mut output = vec![0.0; self.n];

        for i in 0..self.n {
            for j in 0..self.n {
                output[i] += self[(i, j)] * vector[j];
            }
        }

        Vector::new(output)
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, matrix: Matrix) -> Self::Output {
        let mut output = Vec::new();

        for i in 0..self.n {
            let mut row = vec![0.0; self.n];

            for j in 0..self.n {
                for k in 0..self.n {
                    row[j] += self[(i, k)] * matrix[(k, j)];
                }
            }

            output.push(row);
        }

        Matrix::new(output)
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.values[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.values[i]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.values[i.0][i.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut Self::Output {
        &mut self.values[i.0][i.1]
    }
}