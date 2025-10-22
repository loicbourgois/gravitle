import math

A = (0.0, 0.0)
B = (0.0, 1.0)

# Side length
side = math.dist(A, B)

# Midpoint of AB
midpoint = ((A[0] + B[0]) / 2, (A[1] + B[1]) / 2)

# Height of equilateral triangle
height = (math.sqrt(3) / 2) * side

# Vector perpendicular to AB
dx = B[1] - A[1]  # 1
dy = A[0] - B[0]  # 0

# Normalize the perpendicular vector
length = math.hypot(dx, dy)
perp_unit = (dx / length, dy / length)

# Two possible positions for C
C1 = (midpoint[0] + perp_unit[0] * height, midpoint[1] + perp_unit[1] * height)
C2 = (midpoint[0] - perp_unit[0] * height, midpoint[1] - perp_unit[1] * height)

print("Vertex C options:")
print("C1:", C1)
print("C2:", C2)
print(math.sqrt(3) / 2)