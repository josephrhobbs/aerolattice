//! Airframe implementation.
//!
//! The `Airframe` data structure represents a lattice of span-wise airframe sections.
//! The AeroLattice program is designed to perform inviscid fluid flow computations
//! using this data structure.

use pyo3::prelude::*;

use crate::{
    Rib,
    Section,
    Solution,
    Matrix,
    Vector,
    Vector3D,
};

#[pyclass]
#[derive(Clone, Debug)]
/// A lattice of sections representing an aircraft geometry.
pub struct Airframe {
    /// Angle of attack (radians).
    aoa: f64,

    /// Sideslip (radians).
    sideslip: f64,

    #[allow(dead_code)]
    #[pyo3(get, set)]
    /// Reference chord.
    c_ref: f64,

    #[pyo3(get, set)]
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
        // List of sections (built using ribs)
        let mut sections = Vec::new();

        // Number of span-wise vortices per section
        let s = span_count / (ribs.len() - 1);

        for i in 0..(ribs.len() - 1) {
            let r1 = ribs[i];
            let r2 = ribs[i + 1];

            // Function to linearly interpolate chord between R1 and R2
            let interp_chord = |j: f64| r1.chord + (r2.chord - r1.chord) * j / (s as f64);

            // Function to linearly interpolate angle of attack between R1 and R2
            let interp_aoa =   |j: f64| r1.aoa + (r2.aoa - r1.aoa) * j / (s as f64);

            for j in 0..s {
                // Construct P1 and P2 for this section
                let p1 = r1.p + (r2.p - r1.p).scale((j as f64) / (s as f64));
                let p2 = r1.p + (r2.p - r1.p).scale(((j + 1) as f64) / (s as f64));

                // Interpolate chord at halfway point
                let section = Section::new(
                    p1,
                    p2,
                    interp_chord((j as f64) + 0.5),
                    interp_aoa((j as f64) + 0.5),
                    chord_count,
                );

                sections.push(section);
            }
        }

        Self {
            aoa: aoa.to_radians(),
            sideslip: sideslip.to_radians(),
            c_ref,
            s_ref,
            sections,
        }
    }

    #[getter]
    /// Get the angle of attack in degrees.
    pub fn get_aoa(&self) -> f64 {
        self.aoa.to_degrees()
    }

    #[getter]
    /// Get the sideslip in degrees.
    pub fn get_sideslip(&self) -> f64 {
        self.sideslip.to_degrees()
    }

    #[setter]
    /// Set the angle of attack in degrees.
    pub fn set_aoa(&mut self, aoa_deg: f64) {
        self.aoa = aoa_deg.to_radians();
    }

    #[setter]
    /// Set the sideslip in degrees.
    pub fn set_sideslip(&mut self, sideslip_deg: f64) {
        self.sideslip = sideslip_deg.to_radians();
    }

    /// Determine the freestream velocity vector.
    pub fn freestream(&self) -> Vector3D {
        Vector3D::new(
            self.sideslip.cos() * self.aoa.cos(),
            -self.sideslip.sin() * self.aoa.cos(),
            self.aoa.sin(),
        )
    }

    /// Determine total flow at a given point, including vorticity
    ///     effects as well as freestream effects.
    pub fn flow(&self, point: Vector3D) -> Vector3D {
        // Create new output velocity vector
        let mut output = self.freestream();

        // Account for each section
        for section in &self.sections {
            for vortex in &section.vortices {
                output = output + vortex.induced_flow(point);
            }
        }

        output
    }

    /// Construct a Python-compatible solution structure.
    pub fn solve(&self) -> Solution {
        let mut chords = Vec::new();
        let mut spans = Vec::new();
        let mut angles = Vec::new();

        for s in &self.sections {
            chords.push(s.chord);
            spans.push(s.span);
            angles.push(self.aoa + s.aoa);
        }

        Solution::new(
            self.spanwise_coords(),
            Vector::new(chords),
            Vector::new(spans),
            Vector::new(angles),
            self.vorticity_distr(),
            self.s_ref,
        )
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

                // Local angle of attack
                let local_aoa = self.aoa + self.sections[i].aoa;

                let local_freestream = Vector3D::new(
                    self.sideslip.cos() * local_aoa.cos(),
                    -self.sideslip.sin() * local_aoa.cos(),
                    self.aoa.sin(),
                );

                // Component of local freestream in direction of normal
                let component = local_freestream.dot(self.sections[i].normal);

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