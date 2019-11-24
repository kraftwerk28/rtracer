use num::traits::{Float, PrimInt as Int, Zero};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn from_vec(v: Vec<T>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
    pub fn dummy() -> Self {
        Self {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
        }
    }
    pub fn len(&self) -> T {
        Float::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
    pub fn dot(self, v: Vec3<T>) -> T {
        self.x * v.x + self.y + v.y + self.z * v.z
    }
    pub fn vec_prod(a: Self, b: Self) -> Self {
        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;
        Self { x, y, z }
    }
    pub fn norm(&self) -> Self {
        let l = self.len();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Self;
    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, v: T) -> Self {
        Self {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, v: T) -> Self {
        Self {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        }
    }
}

impl Display for Vec3<f64> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( x = {}, y = {}, z = {})", self.x, self.y, self.z)
    }
}

impl Debug for Vec3<f64> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "( x = {}, y = {}, z = {})", self.x, self.y, self.z)
    }
}

impl<T: Float> PartialEq for Vec3<T> {
    fn eq(&self, v: &Self) -> bool {
        self.x == v.x && self.y == v.y && self.z == v.z
    }
}

impl<T: Float> Eq for Vec3<T> {}
