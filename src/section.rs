//! Airframe section implementation.
//!
//! The `Section` data structure represents a span-wise element of an airframe.
//! The AeroLattice program represents a section as one or more horseshoe vortices
//! shed downstream from the quarter-chord of the section.
//!
//! *Note* that at this time, the `Section` data structure only supports one
//! chord-wise horseshoe vortices, but functionality for multiple vortices will
//! be added in the near future.

use pyo3::prelude::*;

use crate::{
    Vector3D,
    VortexPanel,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// An airframe section represented by one or more horseshoe vortices.
pub struct Section {
    /// Leading edge P1.
    p1: Vector3D,

    /// Leading edge P2.
    p2: Vector3D,

    /// Normal vector.
    pub normal: Vector3D,

    /// Boundary condition location.
    pub boundary_condition: Vector3D,

    /// Horseshoe vortex panel.
    vortex: VortexPanel,

    /// Chord-wise dimension of this section.
    /// 
    /// Note that multiple vortex panels may be used in the chord-wise
    /// direction on a single section to create a higher-fidelity simulation.
    chord: f64,
}

#[pymethods]
impl Section {
    #[new]
    /// Construct a new airframe section.
    pub fn new(p1: Vector3D, p2: Vector3D, chord: f64) -> Self {
        let quarter_chord = Vector3D::new(
            0.25 * chord,
            0.0,
            0.0,
        );

        // Vortex with circulation of one (will be changed later)
        let vortex = VortexPanel::new(
            p1 + quarter_chord,
            p2 + quarter_chord,
            1.0,
        );

        // Boundary condition location (half chord from center)
        let boundary_condition = p1 + (p2 - p1).scale(0.5) + quarter_chord.scale(2.0);

        Self {
            p1,
            p2,
            vortex,
            normal: Vector3D::new(0.0, 0.0, 1.0),
            boundary_condition,
            chord,
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this section.
    pub fn py_repr(&self) -> String {
        format!(
            "Section(p1={}, p2={}, chord={})",
            self.p1.py_repr(),
            self.p2.py_repr(),
            self.chord,
        )
    }

    /// Calculate the induced flow of this section at a given point.
    pub fn induced_flow(&self, point: Vector3D) -> Vector3D {
        self.vortex.induced_flow(point)
    }
}