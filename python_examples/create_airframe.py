# AeroLattice Test
# Airframe Creation

import aerolattice as al
import math as m

from matplotlib import pyplot as plt

# Create ribs
r1 = al.Rib(
    al.Vector3D(2, -5, 2),
    chord=0.5,
    aoa=0,
)
r2 = al.Rib(
    al.Vector3D(1, -4, 1),
    chord=1,
    aoa=0,
)
r3 = al.Rib(
    al.Vector3D(0, 0, 0),
    chord=1,
    aoa=0,
)
r4 = al.Rib(
    al.Vector3D(1, 4, 1),
    chord=1,
    aoa=0,
)
r5 = al.Rib(
    al.Vector3D(2, 5, 2),
    chord=0.5,
    aoa=0,
)

angles = [-2, -1, 0, 1, 2, 3, 4]

# Create airframe
airframe = al.Airframe(
    aoa=0,
    sideslip=0,
    c_ref=1,
    s_ref=10,
    span_count=20,
    chord_count=10,
    ribs=[
        r1,
        r2,
        r3,
        r4,
        r5,
    ]
)

lifts = []

for i, a in enumerate(angles):
    airframe.aoa = a

    # Solve airframe
    solution = airframe.solve()

    lifts.append(solution.lift_distr)

for a, l in zip(angles, lifts):
    plt.plot(*l, label=f"AoA = {a} deg")

plt.legend()
plt.xlabel("Spanwise (m)")
plt.ylabel("Sectional Lift")
plt.show()