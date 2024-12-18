use crate::utility::{random_from_range, CLOSEST_TO_ZERO_TO_ONE_RANGE};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3::new(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn unit_vector(v: &Vector3) -> Vector3 {
    v / v.length()
}

pub fn random_unit_vector() -> Vector3 {
    loop {
        let p = Vector3::random_from_range(-1.0..1.0);
        let len_sq = p.length_squared();
        if 1e-160_f64 < len_sq && len_sq < 1.0 {
            return p;
        }
    }
}

pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn reflect(v: &Vector3, normal: &Vector3) -> Vector3 {
    v - 2.0 * dot(&v, &normal) * normal
}

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn random() -> Self {
        Self {
            x: random_from_range(CLOSEST_TO_ZERO_TO_ONE_RANGE),
            y: random_from_range(CLOSEST_TO_ZERO_TO_ONE_RANGE),
            z: random_from_range(CLOSEST_TO_ZERO_TO_ONE_RANGE),
        }
    }

    pub fn random_from_range(range: Range<f64>) -> Self {
        Self {
            x: random_from_range(range.clone()),
            y: random_from_range(range.clone()),
            z: random_from_range(range),
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn normalize(&mut self) -> &Self {
        let l = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        self
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Default for Vector3 {
    fn default() -> Vector3 {
        Vector3::zero()
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl From<(f64, f64, f64)> for Vector3 {
    fn from(tuple: (f64, f64, f64)) -> Vector3 {
        Vector3::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

// Consuming operators (move self)
impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<Vector3> for f64 {
    type Output = Vector3;
    fn div(self, other: Vector3) -> Vector3 {
        Vector3::new(self / other.x, self / other.y, self / other.z)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        Vector3::new(self.x / other, self.y / other, self.z / other)
    }
}

// Non-consuming operators (borrow self)
impl Add for &Vector3 {
    type Output = Vector3;
    fn add(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for &Vector3 {
    type Output = Vector3;
    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul for &Vector3 {
    type Output = Vector3;
    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<&Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Div for &Vector3 {
    type Output = Vector3;
    fn div(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Div<&Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, other: &Vector3) -> Vector3 {
        Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: &Vector3) -> Vector3 {
        Vector3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        Vector3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Div<&Vector3> for f64 {
    type Output = Vector3;
    fn div(self, other: &Vector3) -> Vector3 {
        Vector3::new(self / other.x, self / other.y, self / other.z)
    }
}

// Assignment operators
impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, other: Vector3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
