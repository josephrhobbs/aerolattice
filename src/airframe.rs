//! Airframe implementation.
//!
//! The `Airframe` data structure represents a lattice of span-wise airframe sections.
//! The AeroLattice program is designed to perform inviscid fluid flow computations
//! using this data structure.

use pyo3::prelude::*;

use crate::{
    Rib,
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

    #[allow(dead_code)]
    /// Reference chord.
    c_ref: f64,

    /// Reference planform area.
    s_ref: f64,

    /// List of airframe sections.
    sections: Vec<Section>,
}

#[pymethods]
impl Airframe {
    #[new]
    /// Construct a new airframe from two or more ribs.
    /// 
    /// Note that AoA and sideslip should be set in *degrees*.
    pub fn new(
        aoa: f64,
        sideslip: f64,
        c_ref: f64,
        s_ref: f64,
        span_count: usize,
        chord_count: usize,
        ribs: Vec<Rib>,
    ) -> Self {
        let freestream = Vector3D::new(
            sideslip.to_radians().cos() * aoa.to_radians().cos(),
            -sideslip.to_radians().sin() * aoa.to_radians().cos(),
            aoa.to_radians().sin(),
        );

        // List of sections (built using ribs)
        let mut sections = Vec::new();

        // Number of span-wise vortices per section
        let s = span_count / (ribs.len() - 1);

        for i in 0..(ribs.len() - 1) {
            let r1 = ribs[i];
            let r2 = ribs[i + 1];

            // Function to linearly interpolate chord between R1 and R2
            let interp_chord = |j: f64| r1.chord + (r2.chord - r1.chord) * j / (s as f64);

            for j in 0..s {
                // Construct P1 and P2 for this section
                let p1 = r1.p + (r2.p - r1.p).scale((j as f64) / (s as f64));
                let p2 = r1.p + (r2.p - r1.p).scale(((j + 1) as f64) / (s as f64));

                // Interpolate chord at halfway point
                let section = Section::new(
                    p1,
                    p2,
                    interp_chord((j as f64) + 0.5),
                    chord_count,
                );

                sections.push(section);
            }
        }

        Self {
            freestream,
            c_ref,
            s_ref,
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
            for vortex in &section.vortices {
                output = output + vortex.induced_flow(point);
            }
        }

        output
    }

    /// Solve for the mean aerodynamic chord of this airframe.
    pub fn mac(&self) -> f64 {
        let mut total = 0.0;

        for s in &self.sections {
            total += s.chord;
        }

        total / (self.sections.len() as f64)
    }

    /// Solve for the CL of this airframe.
    pub fn cl(&self) -> f64 {
        // Total lift force, normalized by dynamic pressure
        let mut force = 0.0;

        // Lift distribution (c CL)
        let lift = self.vorticity_distr().scale(2.0).values;

        for i in 0..lift.len() {
            force += lift[i] * self.sections[i].span;
        }

        force / self.s_ref
    }

    /// Solve for the lift coefficient distribution on this airframe.
    /// 
    /// This function returns (non-dimensionalized) lift coefficients.
    pub fn cl_distr(&self) -> (Vec<f64>, Vec<f64>) {
        // Raw values, these need to be normalized by chord length
        let mut output = self.vorticity_distr().scale(2.0).values;

        for i in 0..output.len() {
            // Non-dimensionalize by chord
            output[i] /= self.sections[i].chord;
        }

        (self.spanwise_coords().values, output)
    }

    /// Solve for the sectional lift distribution on this airframe.
    /// 
    /// This function returns lift per unit span.
    pub fn lift_distr(&self) -> (Vec<f64>, Vec<f64>) {
        (self.spanwise_coords().values, self.vorticity_distr().scale(2.0).values)
    }
}

impl Airframe {
    /// Build the normalwash matrix for this airframe.
    fn normalwash_matrix(&self) -> Matrix {
        let mut matrix = Vec::new();

        // For each vortex panel...
        for i in 0..self.sections.len() {
            for j in 0..self.sections[i].vortices.len() {
                // ...evaluate every other panel's contribution to its own normalwash
                let mut row = Vec::new();

                for m in 0..self.sections.len() {
                    for n in 0..self.sections[m].vortices.len() {
                        // Contribution from section `m`, panel `n` towards downwash on section `i`, panel `j`
                        let contribution = self.sections[m].vortices[n].induced_flow(self.sections[i].boundary_conditions[j]);
    
                        // Evaluate normalwash
                        let normalwash = contribution.dot(self.sections[i].normal);
    
                        row.push(normalwash);
                    }
                }
    
                matrix.push(row);
            }
        }

        Matrix::new(matrix)
    }

    /// Build the freestream vector for this airframe.
    fn freestream_vector(&self) -> Vector {
        let mut vector = Vec::new();

        // For each panel...
        for i in 0..self.sections.len() {
            for _ in 0..self.sections[i].vortices.len() {
                // ...evaluate the dot product between the freestream and its section's normal
                let component = self.freestream.dot(self.sections[i].normal);

                // We negate this because it's supposed to be added to the other velocities
                // calculated above, but this is on the other side of the equation
                vector.push(-1.0 * component);
            }
        }

        Vector::new(vector)
    }

    /// Compute the span-wise coordinates at which lift is evaluated.
    fn spanwise_coords(&self) -> Vector {
        let mut vector = Vec::new();

        for i in 0..self.sections.len() {
            vector.push(self.sections[i].center.y);
        }

        Vector::new(vector)
    }

    /// Solve this airframe, returning the vorticity distribution.
    fn vorticity_distr(&self) -> Vector {
        // Raw values, these need to be aggregated by chord-wise coordinate
        let raw_values = self.normalwash_matrix().inverse() * self.freestream_vector();

        let mut output = vec![0.0; self.sections.len()];

        let mut idx = 0;

        for i in 0..self.sections.len() {
            for _ in 0..self.sections[i].vortices.len() {
                output[i] += raw_values[idx];
                idx += 1;
            }
        }

        Vector::new(output)
    }
}