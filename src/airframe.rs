//! Airframe implementation.
//!
//! The `Airframe` data structure represents a lattice of vortex panels.  The
//! AeroLattice program is designed to perform inviscid fluid flow computations
//! using this data structure.

use pyo3::prelude::*;

use crate::{
    Vector3D,
    VortexPanel,
};

#[pyclass]
#[derive(Clone, Debug)]
/// A lattice of vortex panels representing an aircraft geometry.
pub struct Airframe {
    /// Freestream velocity.
    freestream: Vector3D,

    /// List of vortex panels.
    panels: Vec<VortexPanel>,
}

#[pymethods]
impl Airframe {
    #[new]
    /// Construct a new airframe from one or more panels.
    pub fn new(freestream: Vector3D, panels: Vec<VortexPanel>) -> Self {
        Self {
            freestream,
            panels,
        }
    }

    /// Determine total flow at a given point, including vorticity
    ///     effects as well as freestream effects.
    pub fn flow(&self, point: Vector3D) -> Vector3D {
        // Create new output velocity vector
        let mut output = self.freestream;

        // Account for each panel
        for panel in &self.panels {
            output = output + panel.induced_flow(point);
        }

        output
    }
}