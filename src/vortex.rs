//! Vortex panel implementation.

use pyo3::prelude::*;

use crate::Vector3D;

#[pyclass]
/// A vortex panel.
pub struct VortexPanel {
    #[pyo3(get, set)]
    /// Center of the vortex panel.
    pub center: Vector3D,

    #[pyo3(get, set)]
    /// Panel width.
    pub width: f64,

    #[pyo3(get, set)]
    /// Panel angle of attack (radians).
    pub aoa: f64,

    #[pyo3(get, set)]
    /// Circulation strength.
    pub circulation: f64,
}

#[pymethods]
impl VortexPanel {
    #[new]
    pub fn new(
        center: Vector3D,
        width: f64,
        aoa: f64,
        circulation: f64,
    ) -> Self {
        Self {
            center,
            width,
            aoa,
            circulation,
        }
    }

    #[pyo3(name = "__repr__")]
    /// Display a Pythonic representation of this panel.
    pub fn py_repr(&self) -> String {
        format!(
            "VortexPanel(center={}, width={}, aoa={}, circulation={})",
            self.center.py_repr(),
            self.width,
            self.aoa,
            self.circulation,
        )
    }

    /// Compute the flow induced by this vortex at a given point.
    pub fn induced_flow(&self, point: Vector3D) -> Vector3D {
        todo!()
    }
}