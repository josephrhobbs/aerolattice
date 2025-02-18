//! Airframe section implementation.
//!
//! The `Section` data structure represents a span-wise element of an airframe.
//! The AeroLattice program represents a section as one or more horseshoe vortices
//! shed downstream from the section.

use crate::{
    Vector3D,
    VortexPanel,
};

#[derive(Clone, Debug)]
/// An airframe section represented by one or more horseshoe vortices.
pub struct Section {
    /// Center of leading edge.
    pub center: Vector3D,

    /// Horseshoe vortex panels.
    pub vortices: Vec<VortexPanel>,

    /// Boundary condition locations.
    pub boundary_conditions: Vec<Vector3D>,

    /// Normal vector.
    pub normal: Vector3D,

    /// Angle of attack (radians).
    pub aoa: f64,

    /// Span-wise dimension of this section.
    pub span: f64,

    /// Chord-wise dimension of this section.
    /// 
    /// Note that multiple vortex panels may be used in the chord-wise
    /// direction on a single section to create a higher-fidelity simulation.
    pub chord: f64,
}

impl Section {
    /// Construct a new airframe section.
    pub fn new(p1: Vector3D, p2: Vector3D, chord: f64, aoa: f64, count: usize) -> Self {
        // Floating-point equivalent of `count`
        let n = count as f64;

        // Chord (plus vector direction) of a single vortex panel
        let chord_vector = Vector3D::new(
            chord / n,
            0.0,
            0.0,
        );

        // Center of leading edge
        let center = p1 + (p2 - p1).scale(0.5);

        // Normal vector of section
        let downstream = Vector3D::new(
            1.0,
            0.0,
            0.0,
        );
        let normal = downstream.cross(p2 - p1).normalize();

        // Lists of vortices and BCs
        let mut vortices = Vec::new();
        let mut boundary_conditions = Vec::new();

        for i in 0..count {
            // Vortex at this chordwise position (with unit circulation)
            let vortex = VortexPanel::new(
                p1 + chord_vector.scale(0.25 + (i as f64) * n),
                p2 + chord_vector.scale(0.25 + (i as f64) * n),
                1.0,
            );

            // Boundary condition at this location (three-quarters chord)
            let boundary_condition = center + chord_vector.scale(0.75 + (i as f64) * n);

            vortices.push(vortex);
            boundary_conditions.push(boundary_condition);
        }

        let span = (p2 - p1).y;

        Self {
            center,
            vortices,
            boundary_conditions,
            aoa,
            normal,
            span,
            chord,
        }
    }
}