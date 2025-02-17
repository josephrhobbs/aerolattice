# AeroLattice Test
# Airframe Creation

import aerolattice as al
import math as m

from matplotlib import pyplot as plt

# Create control points
p1 = al.Vector3D(0, -5, 0)
p2 = al.Vector3D(0, -4, 0)
p3 = al.Vector3D(0, -3, 0)
p4 = al.Vector3D(0, -2, 0)
p5 = al.Vector3D(0, -1, 0)
p6 = al.Vector3D(0, 0, 0)
p7 = al.Vector3D(0, 1, 0)
p8 = al.Vector3D(0, 2, 0)
p9 = al.Vector3D(0, 3, 0)
p10 = al.Vector3D(0, 4, 0)
p11 = al.Vector3D(0, 5, 0)

# Create sections
section1 = al.Section(p1, p2,    chord=0.5 )
section2 = al.Section(p2, p3,    chord=0.8 )
section3 = al.Section(p3, p4,    chord=1.2 )
section4 = al.Section(p4, p5,    chord=1.2 )
section5 = al.Section(p5, p6,    chord=1.2 )
section6 = al.Section(p6, p7,    chord=1.2 )
section7 = al.Section(p7, p8,    chord=1.2 )
section8 = al.Section(p8, p9,    chord=1.2 )
section9 = al.Section(p9, p10,   chord=0.8 )
section10 = al.Section(p10, p11, chord=0.5 )

# Create airframe
airframe = al.Airframe(
    aoa=1,
    sideslip=0,
    sections=[
        section1,
        section2,
        section3,
        section4,
        section5,
        section6,
        section7,
        section8,
        section9,
        section10,
    ],
)

# Solve airframe
lift_distr = airframe.lift_distr()

print(lift_distr)

plt.plot(*lift_distr, label="c CL")

plt.show()