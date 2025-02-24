//! Solution structure implementation.
//!
//! The `Solution` data structure represents a solution to a vortex lattice model.
//! This structure is designed for accessing aerodynamic coefficients and distributions
//! without resolving for vorticities.

use pyo3::prelude::*;

use crate::{
    Vector,
    Vector3D,
};

/// Pi.
const PI: f64 = std::f64::consts::PI;

#[pyclass]
pub struct Solution {
    /// Span-wise coordinates.
    coordinates: Vector,

    /// Reference planform area.
    s_ref: f64,

    /// Reference span.
    b_ref: f64,

    /// Local section spanwise dimensions.
    spans: Vector,

    /// Local chord lengths.
    chords: Vector,

    /// Local induced angles of attack (radians).
    induced_angles: Vector,

    /// Local normal vectors.
    normals: Vec<Vector3D>,

    /// Overall angle of attack (radians).
    aoa: f64,

    /// Circulation strengths.
    circulations: Vector,
}

#[pymethods]
impl Solution {
    #[getter]
    /// Solve for the mean aerodynamic chord of this airframe.
    pub fn get_mac(&self) -> f64 {
        let mut total = 0.0;

        for c in &self.chords.values {
            total += c;
        }

        total / (self.chords.values.len() as f64)
    }

    #[getter]
    /// Solve for the total aerodynamic force on this airframe.
    pub fn get_aero_force(&self) -> Vector3D {
        // Total lift force, normalized by dynamic pressure
        let mut force = Vector3D::new(0.0, 0.0, 0.0);

        // Circulation values
        let circulations = &self.circulations.values;

        for i in 0..circulations.len() {
            // Normal vector of this section
            let normal = self.normals[i];

            // Effective angle of attack (positive backwards)
            let aoa_eff: f64 = self.aoa - self.induced_angles[i];

            // Force direction (rotate lift vector by effective angle of attack)
            let force_dir = Vector3D::new(
                aoa_eff.cos() * normal.x - aoa_eff.sin() * normal.z,
                normal.y,
                aoa_eff.sin() * normal.x + aoa_eff.cos() * normal.z,
            );

            // Magnitude of circulation
            // Normalized by dynamic pressure and span-wise dimension
            let circulation_magnitude = circulations[i];

            // Span-wise dimension
            let span_dim = self.spans[i];

            // Force contribution
            force = force + force_dir.scale(2.0 * circulation_magnitude * span_dim);
        }

        force
    }

    #[getter]
    /// Solve for the CL (coefficient of lift) of this airframe.
    pub fn get_cl(&self) -> f64 {
        // Total lift force, normalized by dynamic pressure
        let mut force = 0.0;

        // Lift distribution (c cl)
        let lift = self.circulations.scale(2.0).values;

        for i in 0..lift.len() {
            // Local vector perpendicular to flow
            let perpendicular = Vector3D::new(
                -self.aoa.sin(),
                0.0,
                self.aoa.cos(),
            );

            // Cosine of angle between normal and perpendicular vector
            let cos_angle = self.normals[i].dot(perpendicular);

            force += lift[i] * self.spans[i] * self.induced_angles[i].cos() * cos_angle;
        }

        force / self.s_ref
    }

    #[getter]
    /// Solve for the CDi (coefficient of induced drag) of this airframe.
    pub fn get_cdi(&self) -> f64 {
        // Total lift force, normalized by dynamic pressure
        let mut force = 0.0;

        // Lift distribution (c cl)
        let lift = self.circulations.scale(2.0).values;

        for i in 0..lift.len() {   
            force += lift[i] * self.spans[i] * self.induced_angles[i].sin();
        }

        force / self.s_ref
    }

    #[getter]
    /// Solve for the span efficiency coefficient of this airframe.
    pub fn get_span_eff(&self) -> f64 {
        // Lift coefficient
        let cl = self.get_cl();

        // Induced drag coefficient
        let cdi = self.get_cdi();

        // Calculate aspect ratio
        let ar = self.b_ref.powi(2) / self.s_ref;

        cl.powi(2) / PI / cdi / ar
    }

    #[getter]
    /// Solve for the lift coefficient distribution on this airframe.
    /// 
    /// This function returns (non-dimensionalized) lift coefficients.
    pub fn get_cl_distr(&self) -> (Vec<f64>, Vec<f64>) {
        // Raw values, these need to be normalized by chord length
        let mut output = self.circulations.scale(2.0).values;

        for i in 0..output.len() {
            // Non-dimensionalize by chord
            output[i] /= self.chords[i];
        }

        (self.coordinates.values.clone(), output)
    }

    #[getter]
    /// Solve for the sectional lift distribution on this airframe.
    /// 
    /// This function returns lift per unit span.
    pub fn get_lift_distr(&self) -> (Vec<f64>, Vec<f64>) {
        (self.coordinates.values.clone(), self.circulations.scale(2.0).values)
    }

    #[getter]
    /// Get the induced angle of attack on this airframe.
    /// 
    /// This function returns angles in degrees.
    pub fn get_induced_angles(&self) -> (Vec<f64>, Vec<f64>) {
        (self.coordinates.values.clone(), self.induced_angles.scale(180.0 / PI).values)
    }
}

impl Solution {
    /// Construct a new solution.
    pub fn new(
        coordinates: Vector,
        chords: Vector,
        spans: Vector,
        induced_angles: Vector,
        circulations: Vector,
        normals: Vec<Vector3D>,
        aoa: f64,
        s_ref: f64,
        b_ref: f64,
    ) -> Self {
        Self {
            coordinates,
            chords,
            spans,
            induced_angles,
            circulations,
            normals,
            aoa,
            s_ref,
            b_ref,
        }
    }
}