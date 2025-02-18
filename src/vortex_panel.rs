//! Vortex panel implementation.

use pyo3::prelude::*;

use crate::Vector3D;

/// Pi.
const PI: f64 = std::f64::consts::PI;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// A vortex panel.
pub struct VortexPanel {
    #[pyo3(get, set)]
    /// Control point 1.
    pub p1: Vector3D,

    #[pyo3(get, set)]
    /// Control point 2.
    pub p2: Vector3D,

    #[pyo3(get, set)]
    /// Circulation strength.
    pub circulation: f64,
}

#[pymethods]
impl VortexPanel {
    #[new]
    pub fn new(
        p1: Vector3D,
        p2: Vector3D,
        circulation: f64,
    ) -> Self {
        Self {
            p1,
            p2,
            circulation,
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this panel.
    pub fn py_repr(&self) -> String {
        format!(
            "VortexPanel(p1={}, p2={}, circulation={})",
            self.p1.py_repr(),
            self.p2.py_repr(),
            self.circulation,
        )
    }

    /// Compute the flow induced by this vortex at a given point.
    pub fn induced_flow(&self, p0: Vector3D) -> Vector3D {
        // CENTRAL FILAMENT

        // Downstream and upstream directions
        let downstream = Vector3D::new(1.0, 0.0, 0.0);
        let upstream = downstream.scale(-1.0);

        // Distance from given point to central filament
        let r_central = ((p0 - self.p1) * (p0 - self.p2)).abs() / (self.p2 - self.p1).abs();

        // Direction to closest point
        let t = - (self.p1 - p0).dot(self.p2 - self.p1) / (self.p2 - self.p1).abs().powi(2);
        let closest_point = Vector3D::new(
            self.p1.x + (self.p2.x - self.p1.x) * t,
            self.p1.y + (self.p2.y - self.p1.y) * t,
            self.p1.z + (self.p2.z - self.p1.z) * t,
        );
        let to_central_filament = (closest_point - p0).normalize();

        // Direction to P1 and P2
        let to_p1 = (self.p1 - p0).normalize();
        let to_p2 = (self.p2 - p0).normalize();

        // Unit vector of central filament
        let p1_to_p2 = (self.p2 - self.p1).normalize();

        // Angles
        let cos_angle1 = to_p1.scale(-1.0).dot(p1_to_p2);
        let cos_angle2 = to_p2.scale(-1.0).dot(p1_to_p2);

        // Contribution of central filament
        let contribution_central = -self.circulation / 4.0 / PI / r_central * (cos_angle1 - cos_angle2);
        let v1 = (self.p2 - self.p1).cross(to_central_filament).normalize().scale(contribution_central);

        // LEFT FILAMENT (P1 -> INFTY)
        // The left filament enters from infinity into P1

        // Distance from given point to left filament
        let r_left = ((p0.y - self.p1.y).powi(2) + (p0.z - self.p1.z).powi(2)).sqrt();

        // Direction to closest point
        let projected_p0 = Vector3D::new(
            self.p1.x,
            p0.y,
            p0.z,
        );
        let to_left_filament = (self.p1 - projected_p0).normalize();

        // Angle
        // Note: floating-point precision can make this slightly larger (1e-10) than one,
        // but this breaks f64::acos() so we use f64::min()
        let angle2 = PI/2.0 + to_left_filament.dot(to_p1).min(1.0).acos();

        // Contribution of left filament
        let contribution_left = -self.circulation / 4.0 / PI / r_left * (1.0 - angle2.cos());
        let v2 = upstream.cross(to_left_filament).normalize().scale(contribution_left);

        // RIGHT FILAMENT (P2 -> INFTY)

        // Distance from given point to right filament
        let r_right = ((p0.y - self.p2.y).powi(2) + (p0.z - self.p2.z).powi(2)).sqrt();

        // Direction to closest point
        let projected_p0 = Vector3D::new(
            self.p2.x,
            p0.y,
            p0.z,
        );
        let to_right_filament = (self.p2 - projected_p0).normalize();

        // Angle
        // Note: floating-point precision can make this slightly larger (1e-10) than one,
        // but this breaks f64::acos() so we use f64::min()
        let angle1 = PI/2.0 + to_right_filament.dot(to_p2).min(1.0).acos();

        // Contribution of right filament
        let contribution_right = -self.circulation / 4.0 / PI / r_right * (1.0 - angle1.cos());
        let v3 = downstream.cross(to_right_filament).normalize().scale(contribution_right);

        v1 + v2 + v3
    }
}