//! Main library for AeroLattice.

#[deny(warnings)]
#[deny(missing_docs)]

mod matrix;
mod vector;
mod vector3d;

use pyo3::prelude::*;

pub use matrix::Matrix;

pub use vector::Vector;

pub use vector3d::Vector3D;

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn aerolattice(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    
    // Add `Matrix` class
    m.add_class::<Matrix>()?;

    // Add `Vector` class
    m.add_class::<Vector>()?;

    // Add `Vector3D` class
    m.add_class::<Vector3D>()?;
    
    Ok(())
}
