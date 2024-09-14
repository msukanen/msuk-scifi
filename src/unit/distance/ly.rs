use std::ops::{Add, Div, Mul, Sub};

use super::{au::Au, km::Km, mi::Mi, pc::Pc, Distanced};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Ly {
    value: f64,
}

impl Distanced for Ly {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Ly {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Au> for Ly {
    fn from(value: Au) -> Self {
        Self { value: value.raw_value() / 63241.0771 }
    }
}

impl From<Km> for Ly {
    fn from(value: Km) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Mi> for Ly {
    fn from(value: Mi) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Pc> for Ly {
    fn from(value: Pc) -> Self {
        Self { value: value.raw_value() * 3.26156378 }
    }
}

impl std::fmt::Display for Ly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ly", self.value)
    }
}

impl Mul<f64> for Ly {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: self.value * rhs }
    }
}

impl Add<Ly> for f64 {
    type Output = Ly;
    fn add(self, rhs: Ly) -> Self::Output {
        rhs + self
    }
}

impl Add<f64> for Ly {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self { value: self.value + rhs }
    }
}

impl Sub<Ly> for f64 {
    type Output = Ly;
    fn sub(self, rhs: Ly) -> Self::Output {
        Ly { value: self - rhs.value }
    }
}

impl Sub<f64> for Ly {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self { value: self.value - rhs}
    }
}

impl Div<f64> for Ly {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { value: self.value / rhs }
    }
}

impl Div<Ly> for f64 {
    type Output = Ly;
    fn div(self, rhs: Ly) -> Self::Output {
        Ly::from( self / rhs.value )
    }
}

impl Ly {
    pub fn sqrt(&self) -> Self {
        Self { value: self.value.sqrt() }
    }
}
