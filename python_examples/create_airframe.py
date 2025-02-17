# AeroLattice Test
# Airframe Creation

import aerolattice as al

# Create control points
p1 = al.Vector3D(0, -1, 0)
p2 = al.Vector3D(0, 0, 0)
p3 = al.Vector3D(0, 1, 0)

# Create vortex panels
panel1 = al.VortexPanel(p1, p2, circulation=1)
panel2 = al.VortexPanel(p2, p3, circulation=1)

# Create airframe
airframe = al.Airframe(
    freestream=al.Vector3D(50, 0, 0),
    panels=[panel1, panel2],
)