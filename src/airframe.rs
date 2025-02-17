//! Airframe implementation.
//!
//! The `Airframe` data structure represents a lattice of span-wise airframe sections.
//! The AeroLattice program is designed to perform inviscid fluid flow computations
//! using this data structure.

use pyo3::prelude::*;

use crate::{
    Section,
    Matrix,
    Vector,
    Vector3D,
};

#[pyclass]
#[derive(Clone, Debug)]
/// A lattice of sections representing an aircraft geometry.
pub struct Airframe {
    /// Freestream velocity vector.
    freestream: Vector3D,

    /// List of airframe sections.
    sections: Vec<Section>,
}

#[pymethods]
impl Airframe {
    #[new]
    /// Construct a new airframe from one or more sections.
    /// 
    /// Note that AoA and sideslip should be set in *degrees*.
    pub fn new(aoa: f64, sideslip: f64, sections: Vec<Section>) -> Self {
        let freestream = Vector3D::new(
            sideslip.to_radians().cos() * aoa.to_radians().cos(),
            -sideslip.to_radians().sin() * aoa.to_radians().cos(),
            aoa.to_radians().sin(),
        );

        Self {
            freestream,
            sections,
        }
    }

    /// Determine total flow at a given point, including vorticity
    ///     effects as well as freestream effects.
    pub fn flow(&self, point: Vector3D) -> Vector3D {
        // Create new output velocity vector
        let mut output = self.freestream;

        // Account for each section
        for section in &self.sections {
            output = output + section.induced_flow(point);
        }

        output
    }

    /// Solve this airframe, returning the vorticity distribution.
    pub fn vorticity(&self) -> Vector {
        self.normalwash_matrix().inverse() * self.freestream_vector()
    }

    /// Solve for the sectional lift distribution on this airframe.
    /// 
    /// This function returns lift per unit span.
    pub fn lift_distr(&self) -> (Vec<f64>, Vec<f64>) {
        (self.spanwise_coords().values, self.vorticity().scale(2.0).values)
    }
}

impl Airframe {
    /// Build the normalwash matrix for this airframe.
    fn normalwash_matrix(&self) -> Matrix {
        let mut matrix = Vec::new();

        // For each section...
        for i in 0..self.sections.len() {
            // ...evaluate every other section's contribution to its own normalwash
            let mut row = Vec::new();

            for j in 0..self.sections.len() {
                // Contribution from section `j` towards downwash on section `i`
                let contribution = self.sections[j].induced_flow(self.sections[i].boundary_condition);

                // Evaluate normalwash
                let normalwash = contribution.dot(self.sections[i].normal);

                row.push(normalwash);
            }

            matrix.push(row);
        }

        Matrix::new(matrix)
    }

    /// Build the freestream vector for this airframe.
    fn freestream_vector(&self) -> Vector {
        let mut vector = Vec::new();

        // For each section...
        for i in 0..self.sections.len() {
            // ...evaluate the dot product between the freestream and this section's normal
            let component = self.freestream.dot(self.sections[i].normal);

            // We negate this because it's supposed to be added to the other velocities
            // calculated above, but this is on the other side of the equation
            vector.push(-1.0 * component);
        }

        Vector::new(vector)
    }

    /// Compute the span-wise coordinates at which lift is evaluated.
    fn spanwise_coords(&self) -> Vector {
        let mut vector = Vec::new();

        for i in 0..self.sections.len() {
            vector.push(self.sections[i].boundary_condition.y);
        }

        Vector::new(vector)
    }
}