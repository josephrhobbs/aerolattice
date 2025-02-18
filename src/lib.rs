//! Main library for AeroLattice.

#[deny(warnings)]
#[deny(missing_docs)]

mod airframe;
mod matrix;
mod rib;
mod solution;
mod section;
mod vector;
mod vector3d;
mod vortex_panel;

use pyo3::prelude::*;

pub use airframe::Airframe;

pub use matrix::Matrix;

pub use rib::Rib;

pub use section::Section;

pub use solution::Solution;

pub use vector::Vector;

pub use vector3d::Vector3D;

pub use vortex_panel::VortexPanel;

/// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

/// A Python module implemented in Rust.
#[pymodule]
fn aerolattice(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    
    // Add `Airframe` class
    m.add_class::<Airframe>()?;

    // Add `Matrix` class
    m.add_class::<Matrix>()?;

    // Add `Rib` class
    m.add_class::<Rib>()?;

    // Add `Solution` class
    m.add_class::<Solution>()?;

    // Add `Vector` class
    m.add_class::<Vector>()?;

    // Add `Vector3D` class
    m.add_class::<Vector3D>()?;

    // Add `VortexPanel` class
    m.add_class::<VortexPanel>()?;
    
    Ok(())
}
