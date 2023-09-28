use std::ops::*;
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Vec3(f64, f64, f64);
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self{ 0: self.0 + other.0, 1: self.1 + other.1, 2: self.2 + other.2 }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self{ 0: self.0 - other.0, 1: self.1 - other.1, 2: self.2 - other.2 }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, other: f64) -> Self {
        Self{ 0: self.0 * other, 1: self.1 * other, 2: self.2 * other }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Self {
        Self{ 0: self.0 * other.0, 1: self.1 * other.1, 2: self.2 * other.2 }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, other: f64) -> Self {
        Self{ 0: self.0/other, 1: self.1/other, 2: self.2/other }
    }
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{ 0: x, 1: y, 2: z }
    }
    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        loop {
            let vec = Vec3{ 0: rng.gen_range(-1.0, 1.0), 1: rng.gen_range(-1.0, 1.0), 2: rng.gen_range(-1.0, 1.0)};
            if vec.len() >= 1.0 {
                return vec;
            }
        } 
    }
}

pub type Colour = Vec3;
impl Colour {
    pub fn blank() -> Colour {
        Colour{ 0: 0.0, 1: 0.0, 2: 0.0 }
    }
}

impl Colour {
    #[inline]
    pub fn r(&self) -> f64 {
        self.x()
    }
    #[inline]
    pub fn g(&self) -> f64 {
        self.y()
    }
    #[inline]
    pub fn b(&self) -> f64 {
        self.z()
    }
}

pub type Position = Vec3 ;
impl Position {

    #[inline]
    pub fn to_direction(self) -> Direction {
        self
    }
}

pub type Direction = Vec3;
impl Direction {
    #[inline]
    pub fn len(&self) -> f64 {
        (self.0.powf(2.0_f64)+self.1.powf(2.0_f64)+self.2.powf(2.0_f64)).powf(0.5_f64)
    }
    #[inline]
    pub fn dot(&self, other: &Direction) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    #[inline]
    pub fn normalise(&self) -> Direction {
        Direction{ 0: self.0/self.len(), 1: self.1/self.len(), 2: self.2/self.len() }
    }
    #[inline]
    pub fn to_position(self) -> Position {
        self
    }
}

