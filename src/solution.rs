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

#[pyclass]
pub struct Solution {
    /// Span-wise coordinates.
    coordinates: Vector,

    /// Reference planform area.
    s_ref: f64,

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
    /// Solve for the CL (coefficient of lift) of this airframe.
    pub fn get_cl(&self) -> f64 {
        // Total lift force, normalized by dynamic pressure
        let mut force = 0.0;

        // Lift distribution (c CL)
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

        // Lift distribution (c CL)
        let lift = self.circulations.scale(2.0).values;

        for i in 0..lift.len() {   
            force += lift[i] * self.spans[i] * self.induced_angles[i].sin();
        }

        force / self.s_ref
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
        }
    }
}