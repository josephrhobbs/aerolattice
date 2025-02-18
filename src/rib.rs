//! Airframe rib implementation.
//!
//! The `Rib` data structure loosely resembles a structural rib in
//! the design of an aircraft.  It represents a "control point" at
//! which the user can define the leading edge location and chord
//! length.  The AeroLattice program will then construct a series
//! of `Section` data structures along the span-wise coordinate.

use pyo3::prelude::*;

use crate::Vector3D;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// An airframe rib represented by one or more span-wise sections.
pub struct Rib {
    /// Leading edge point.
    pub p: Vector3D,

    /// Wing chord at this location.
    pub chord: f64,

    /// Angle of attack (radians).
    pub aoa: f64,
}

#[pymethods]
impl Rib {
    #[new]
    /// Construct a new aircraft rib.
    /// 
    /// *Note* that the angle of attack function parameter must be in *degrees*.
    pub fn new(p: Vector3D, chord: f64, aoa: f64) -> Self {
        Self {
            p,
            chord,
            aoa: aoa.to_radians(),
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this rib.
    pub fn py_repr(&self) -> String {
        format!(
            "Rib(p={}, chord={})",
            self.p.py_repr(),
            self.chord,
        )
    }
}