use std::ops::{Add, Div, Mul, Sub};

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{RgbColor, WebColors},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct PointOrVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PointOrVector {
    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).powf(0.5)
    }

    pub fn unit_vector(self) -> PointOrVector {
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn distance_between(self, other_point: Self) -> f64 {
        (self - other_point).magnitude()
    }

    // TODO: Maybe make a projection fn instead and have unit vector constants
    pub const fn x_only(self) -> Self {
        Self {
            x: self.x,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn y_only(self) -> Self {
        Self {
            x: 0.0,
            y: self.y,
            z: 0.0,
        }
    }

    pub const fn z_only(self) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: self.z,
        }
    }

    /// https://en.wikipedia.org/wiki/Cross_product#Computing
    pub const fn cross_product(self, other_point: Self) -> Self {
        let a = self;
        let b = other_point;
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub const fn dot_product(self, other_vector: Self) -> f64 {
        let a = self;
        let b = other_vector;
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

impl Mul<f64> for PointOrVector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        PointOrVector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for PointOrVector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        PointOrVector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for PointOrVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for PointOrVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub const CAMERA_POS: PointOrVector = PointOrVector {
    x: 10.0,
    y: 0.0,
    z: 0.0,
};
pub const CAMERA_DIRECTION: PointOrVector = PointOrVector {
    x: -1.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: PointOrVector,
    pub direction: PointOrVector,
}

impl Ray {
    pub fn from_start_to_end(start: PointOrVector, end: PointOrVector) -> Self {
        Self {
            start,
            direction: end - start,
        }
    }

    pub fn multiplier_to_plane_intersection(self, plane: Plane) -> Option<f64> {
        let denominator =
            plane.a * self.direction.x + plane.b * self.direction.y + plane.c * self.direction.z;
        if denominator.abs() < 1e-3 {
            None
        } else {
            let multiplier = (plane.d
                - (plane.a * self.start.x + plane.b * self.start.y + plane.c * self.start.z))
                / denominator;
            Some(multiplier)
        }
    }
}

// Define a axis as a vector in one direction of the axis
pub const SCREEN_Y_AXIS: Ray = Ray {
    start: PointOrVector::zero(),
    direction: PointOrVector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    },
};
pub const SCREEN_X_AXIS: Ray = Ray {
    start: PointOrVector::zero(),
    direction: PointOrVector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    },
};

/// Represented as
/// ax + by + cz = d
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

impl Plane {
    /// https://www.quora.com/What-is-the-fastest-way-to-find-the-equation-of-a-plane-given-three-points
    pub fn from_3_points(points: [PointOrVector; 3]) -> Self {
        let [a, b, c] = points;
        let a_b = b - a;
        let a_c = c - a;
        let u = a_b.cross_product(a_c);
        // u.x*x + u.y*y + u.z*z + k = 0
        // u.x*(x-a.x) + u.y(y-a.y) + u.z(y-a.z) + k = 0
        let k = -(u.x * a.x + u.y * a.y + u.z * a.z);
        Self {
            a: u.x,
            b: u.y,
            c: u.z,
            d: -k,
        }
    }
}

pub const PLANE_CONTAINING_X_AXIS_AND_CAMERA: Plane = Plane {
    a: 0.0,
    b: 1.0,
    c: 0.0,
    d: 0.0,
};

pub const PLANE_CONTAINING_Y_AXIS_AND_CAMERA: Plane = Plane {
    a: 0.0,
    b: 0.0,
    c: 1.0,
    d: 0.0,
};

/// https://mathinsight.org/distance_point_plane
pub fn distance_from_point_to_plane(point: PointOrVector, plane: Plane) -> f64 {
    (plane.a * point.x + plane.b * point.y + plane.c * point.z).abs()
        / (plane.a.powi(2) + plane.b.powi(2) + plane.c.powi(2)).powf(0.5)
}

pub fn ray_from_point_to_plane(point: PointOrVector, plane: Plane) -> Ray {
    Ray {
        start: point,
        direction: PointOrVector {
            x: -plane.a,
            y: -plane.b,
            z: -plane.c,
        },
    }
}

pub fn point_of_intersection_of_plane_and_normal_line(
    point: PointOrVector,
    plane: Plane,
) -> PointOrVector {
    let ray = ray_from_point_to_plane(point, plane);
    let distance = distance_from_point_to_plane(point, plane);
    let unit_vector = ray.direction.unit_vector();
    let distance_vector = unit_vector * distance;
    ray.start + distance_vector
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
