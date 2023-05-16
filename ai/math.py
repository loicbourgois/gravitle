import math


f32 = float


class Vector():
    def __init__(self,x=None,y=None):
        assert x is not None
        assert y is not None
        self.x = x
        self.y = y
    
    def __mul__(self, value):
        new_vec = Vector(0,0)
        new_vec.x = self.x * value
        new_vec.y = self.y * value
        return new_vec
    
    def __add__(self, other):
        new_vec = Vector(0,0)
        new_vec.x = self.x + other.x
        new_vec.y = self.y + other.y
        return new_vec


def cross(p1: Vector, p2: Vector):
    return p1.x * p2.y - p1.y * p2.x



class WrapAroundResponse():
    def __init__(self, a=None, b=None, d=None, d_sqrd=None):
        self.a = a
        self.b = b
        self.d = d
        self.d_sqrd = d_sqrd


def delta(a: Vector, b: Vector) -> Vector:
    return Vector(
        b.x - a.x,
        b.y - a.y
    )


def distance_sqrd(a: Vector, b: Vector):
    dp = delta(a, b)
    return dp.x * dp.x + dp.y * dp.y


def normalize_2(p: Vector) -> Vector:
    d = math.sqrt((p.x * p.x + p.y * p.y))
    return Vector(
        x=p.x / d,
        y=p.y / d,
    )


def wrap_around(a: Vector, b: Vector) -> WrapAroundResponse:
    dsqrd_min = distance_sqrd(a, b)
    ijwin = [0.0, 0.0]
    aa = 1.0
    ijs = [
        [-aa, -aa],
        [-aa, 0.0],
        [-aa, aa],
        [0.0, -aa],
        [0.0, aa],
        [aa, -aa],
        [aa, 0.0],
        [aa, aa],
    ]
    for ij in ijs:
        dsqrd = distance_sqrd(
            a,
            Vector(
                b.x + ij[0],
                b.y + ij[1],
            )
        );
        if dsqrd < dsqrd_min:
            dsqrd_min = dsqrd
            ijwin = ij
    bbb = Vector (
        b.x + ijwin[0],
        b.y + ijwin[1],
    )
    aaa = Vector (
        a.x + ijwin[0],
        a.y + ijwin[1],
    )
    d = delta(a, bbb)
    return WrapAroundResponse (
        a= aaa,
        b= bbb,
        d=d,
        d_sqrd= dsqrd_min,
    )

def norm_sqrd(v: Vector):
    return v.x * v.x + v.y * v.y

def norm(v: Vector) -> f32:
    return math.sqrt(norm_sqrd(v))


def degrees(x: f32):
    return x * (180.0 / math.pi)


def angle(p1: Vector, p2: Vector) -> f32:
    cross_ = cross(p1, p2)
    l = norm(p1) * norm(p2)
    angle = math.asin(cross_ / l)
    return degrees(angle)
