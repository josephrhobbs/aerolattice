//! Airframe implementation.
//!
//! The `Airframe` data structure represents a lattice of span-wise airframe sections.
//! The AeroLattice program is designed to perform inviscid fluid flow computations
//! using this data structure.

use pyo3::{
    prelude::*,
    exceptions::PyValueError,
};

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

    #[pyo3(get, set)]
    /// Reference span.
    b_ref: f64,

    #[pyo3(get, set)]
    /// Reference planform area.
    s_ref: f64,

    /// List of airframe sections.
    sections: Vec<Section>,
}

#[pymethods]
impl Airframe {
    #[new]
    #[pyo3(signature=(s_ref, b_ref, ribs, span_count=30, chord_count=10))]
    /// Construct a new airframe from two or more ribs.
    pub fn new(
        s_ref: f64,
        b_ref: f64,
        ribs: Vec<Rib>,
        span_count: usize,
        chord_count: usize,
    ) -> PyResult<Self> {
        // List of sections (built using ribs)
        let mut sections = Vec::new();

        // Number of span-wise vortices per section
        let s = span_count / (ribs.len() - 1);

        // We need at least two ribs
        if ribs.len() < 2 {
            return Err (PyValueError::new_err("two or more airframe ribs are required"));
        }

        for i in 0..(ribs.len() - 1) {
            let r1 = ribs[i];
            let r2 = ribs[i + 1];

            // Function to linearly interpolate chord between R1 and R2
            let interp_chord = |j: f64| r1.chord + (r2.chord - r1.chord) * j / (s as f64);

            // Function to linearly interpolate incidence angle between R1 and R2
            let interp_aoa =   |j: f64| r1.incidence + (r2.incidence - r1.incidence) * j / (s as f64);

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

        Ok (Self {
            aoa: 0.0,
            sideslip: 0.0,
            b_ref,
            s_ref,
            sections,
        })
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
        let mut normals = Vec::new();

        // Calculate the inverse normalwash matrix
        let inverse_normalwash_matrix = self.normalwash_matrix().inverse();

        // Calculate the freestream vector
        let freestream_vector = self.freestream_vector();

        // Calculate the vorticity distribution
        let vorticity_distr = self.vorticity_distr(&inverse_normalwash_matrix, &freestream_vector);
        
        // Calculate the induced AoA distribution
        let induced_angles_distr = self.induced_angles_distr(&inverse_normalwash_matrix, &freestream_vector);

        for i in 0..self.sections.len() {
            let s = &self.sections[i];

            chords.push(s.chord);
            spans.push(s.span);
            normals.push(s.normal);
        }

        Solution::new(
            self.spanwise_coords(),
            Vector::new(chords),
            Vector::new(spans),
            induced_angles_distr,
            vorticity_distr,
            normals,
            self.aoa,
            self.s_ref,
            self.b_ref,
        )
    }
}

impl Airframe {
    /// Build the normalwash matrix for this airframe.
    /// 
    /// The elements of the normalwash matrix have units of inverse distance (velocity per unit circulation).
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

                        // Evaluate normalwash (scalar) from contribution (`Vector3D`)
                        let normalwash = contribution.dot(self.sections[i].normal);

                        row.push(normalwash);
                    }
                }
    
                matrix.push(row);
            }
        }

        Matrix::new(matrix)
    }

    /// Build the normalwash matrix, neglecting bound vorticity, for this airframe.
    fn trailing_normalwash_matrix(&self) -> Matrix {
        let mut matrix = Vec::new();

        // For each vortex panel...
        for i in 0..self.sections.len() {
            for j in 0..self.sections[i].vortices.len() {
                // ...evaluate every other panel's contribution to its own normalwash (neglecting the bound vortex)
                let mut row = Vec::new();

                for m in 0..self.sections.len() {
                    for n in 0..self.sections[m].vortices.len() {
                        // Contribution from section `m`, panel `n` towards downwash on section `i`, panel `j`
                        let contribution = self.sections[m].vortices[n].downwash_flow(self.sections[i].boundary_conditions[j]);

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
    /// 
    /// The elements of the freestream vector are unitless (local velocity per unit freestream velocity).
    fn freestream_vector(&self) -> Vector {
        let mut vector = Vec::new();

        // For each panel...
        for i in 0..self.sections.len() {
            for _ in 0..self.sections[i].vortices.len() {
                // ...evaluate the dot product between the freestream and its section's normal

                // Local angle of attack
                let local_aoa = self.aoa + self.sections[i].incidence;

                let local_freestream = Vector3D::new(
                    self.sideslip.cos() * local_aoa.cos(),
                    -self.sideslip.sin() * local_aoa.cos(),
                    local_aoa.sin(),
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

    /// Build the induced angle of attack vector for this airframe.
    fn induced_angles_distr(&self, inverse_normalwash_matrix: &Matrix, freestream_vector: &Vector) -> Vector {       
        // Compute resultant downwash values
        let result = self.trailing_normalwash_matrix() * inverse_normalwash_matrix.clone() * freestream_vector.clone();

        let mut output = vec![0.0; self.sections.len()];

        let mut idx = 0;

        for i in 0..self.sections.len() {
            // Chord-wise step... we need to average the downwash
            let c = self.sections[i].chord / (self.sections[i].vortices.len() as f64);

            for _ in 0..self.sections[i].vortices.len() {
                // Negative because "downwash" is defined as positive downwards
                output[i] -= result[idx] * c;
                idx += 1;
            }

            // Convert downwash value to induced angle of attack
            output[i] = output[i].atan();
        }

        Vector::new(output)
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
    /// 
    /// The vorticity distribution has units of distance (circulation per unit freestream velocity).
    fn vorticity_distr(&self, inverse_normalwash_matrix: &Matrix, freestream_vector: &Vector) -> Vector {       
        // Raw values, these need to be aggregated by chord-wise coordinate
        let raw_values = inverse_normalwash_matrix.clone() * freestream_vector.clone();

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