use std::ops::{Add, Div, Mul, Sub};

use super::{au::Au, km::Km, ly::Ly, pc::Pc, Distanced};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Mi {
    value: f64,
}

impl Mi {
    pub const KM_IN_MI: f64 = 1.609344;
}

impl Distanced for Mi {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Mi {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Km> for Mi {
    fn from(value: Km) -> Self {
        Self { value: value.raw_value() / Self::KM_IN_MI }
    }
}

impl From<Au> for Mi {
    fn from(value: Au) -> Self {
        Self::from(Km::from(value))
    }
}

impl From<Ly> for Mi {
    fn from(value: Ly) -> Self {
        Self::from(Au::from(value))
    }
}

impl From<Pc> for Mi {
    fn from(value: Pc) -> Self {
        Self::from(Ly::from(value))
    }
}

impl std::fmt::Display for Mi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mi.", self.value)
    }
}

impl Mul<f64> for Mi {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: self.value * rhs }
    }
}

impl Add<Mi> for f64 {
    type Output = Mi;
    fn add(self, rhs: Mi) -> Self::Output {
        rhs + self
    }
}

impl Add<f64> for Mi {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self { value: self.value + rhs }
    }
}

impl Sub<Mi> for f64 {
    type Output = Mi;
    fn sub(self, rhs: Mi) -> Self::Output {
        Mi { value: self - rhs.value }
    }
}

impl Sub<f64> for Mi {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self { value: self.value - rhs}
    }
}

impl Div<f64> for Mi {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { value: self.value / rhs }
    }
}

impl Div<Mi> for f64 {
    type Output = Mi;
    fn div(self, rhs: Mi) -> Self::Output {
        Mi::from( self / rhs.value )
    }
}

impl Mi {
    pub fn sqrt(&self) -> Self {
        Self { value: self.value.sqrt() }
    }
}
