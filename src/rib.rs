//! Airframe rib implementation.
//!
//! The `Rib` data structure loosely resembles a structural rib in
//! the design of an aircraft.  It represents a "control point" at
//! which the user can define the leading edge location and chord
//! length.  The AeroLattice program will then construct a series
//! of `Section` data structures along the span-wise coordinate.

use pyo3::prelude::*;

use crate::{
    Section,
    Vector3D,
};

#[pyclass]
#[derive(Clone, Debug)]
/// An airframe rib represented by one or more span-wise sections.
pub struct Rib {
    /// Leading edge P1.
    p1: Vector3D,

    /// Leading edge P2.
    p2: Vector3D,

    /// Center of leading edge.
    pub center: Vector3D,

    /// Horseshoe vortex panels.
    pub sections: Vec<VortexPanel>,

    /// Boundary condition locations.
    pub boundary_conditions: Vec<Vector3D>,

    /// Normal vector.
    pub normal: Vector3D,

    /// Chord-wise dimension of this section.
    /// 
    /// Note that multiple vortex panels may be used in the chord-wise
    /// direction on a single section to create a higher-fidelity simulation.
    pub chord: f64,
}