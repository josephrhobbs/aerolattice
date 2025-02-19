# AeroLattice Test
# Airframe Creation

import aerolattice as al
import math as m

from matplotlib import pyplot as plt

# Create ribs
r1 = al.Rib(
    al.Vector3D(0.25, -5, 0),
    chord=0.5,
    incidence=0,
)
r2 = al.Rib(
    al.Vector3D(0.1, -2.5, 0),
    chord=0.8,
    incidence=0,
)
r3 = al.Rib(
    al.Vector3D(0, 0, 0),
    chord=1,
    incidence=0,
)
r4 = al.Rib(
    al.Vector3D(0.1, 2.5, 0),
    chord=0.8,
    incidence=0,
)
r5 = al.Rib(
    al.Vector3D(0.25, 5, 0),
    chord=0.5,
    incidence=0,
)

# Create airframe
airframe = al.Airframe(
    c_ref=1,
    s_ref=10,
    span_count=30,
    chord_count=10,
    ribs=[
        r1,
        r2,
        r3,
        r4,
        r5,
    ]
)
airframe.aoa = 5

sol = airframe.solve()

lift = sol.lift_distr
cl   = sol.cl_distr

print(f"CL  = {sol.cl}")
print(f"CDi = {sol.cdi}")
print(f"eff = {sol.cl**2 / 11.7 / m.pi / sol.cdi}")

plt.plot(*lift, label="c cl")
plt.plot(*cl, label="cl")
plt.legend()

plt.xlabel("Spanwise (m)")
plt.ylabel("Sectional Lift")

plt.show()