# AeroLattice Test
# Airframe Creation

import aerolattice as al
import math as m

from matplotlib import pyplot as plt

# Create ribs
r1 = al.Rib(
    al.Vector3D(1, -5, 2),
    chord=0.5,
)
r2 = al.Rib(
    al.Vector3D(0, -4, 1),
    chord=1,
)
r3 = al.Rib(
    al.Vector3D(0, 0, 0),
    chord=1,
)
r4 = al.Rib(
    al.Vector3D(0, 4, 1),
    chord=1,
)
r5 = al.Rib(
    al.Vector3D(1, 5, 2),
    chord=0.5,
)

# Create airframe
airframe = al.Airframe(
    aoa=2,
    sideslip=0,
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

# Solve airframe
lift = airframe.lift_distr()
cl = airframe.cl_distr()

print(lift)
print(f"CL = {airframe.cl()}")

plt.plot(*lift, label="c CL")
plt.plot(*cl, label="CL")
plt.legend()
plt.xlabel("Spanwise (m)")
plt.ylabel("Sectional Lift")
plt.show()