use std::ops::{Add, Div, Mul, Sub};

use super::{au::Au, km::Km, ly::Ly, mi::Mi, Distanced};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Pc {
    value: f64,
}

impl Distanced for Pc {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Pc {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Ly> for Pc {
    fn from(value: Ly) -> Self {
        Self { value: value.raw_value() /  3.26156378 }
    }
}

impl From<Au> for Pc {
    fn from(value: Au) -> Self {
        Self::from(Ly::from(value))
    }
}

impl From<Mi> for Pc {
    fn from(value: Mi) -> Self {
        Self::from(Ly::from(value))
    }
}

impl From<Km> for Pc {
    fn from(value: Km) -> Self {
        Self::from(Ly::from(value))
    }
}

impl std::fmt::Display for Pc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} pc", self.value)
    }
}

impl Mul<f64> for Pc {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: self.value * rhs }
    }
}

impl Add<Pc> for f64 {
    type Output = Pc;
    fn add(self, rhs: Pc) -> Self::Output {
        rhs + self
    }
}

impl Add<f64> for Pc {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self { value: self.value + rhs }
    }
}

impl Sub<Pc> for f64 {
    type Output = Pc;
    fn sub(self, rhs: Pc) -> Self::Output {
        Pc { value: self - rhs.value }
    }
}

impl Sub<f64> for Pc {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self { value: self.value - rhs}
    }
}

impl Div<f64> for Pc {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { value: self.value / rhs }
    }
}

impl Div<Pc> for f64 {
    type Output = Pc;
    fn div(self, rhs: Pc) -> Self::Output {
        Pc::from( self / rhs.value )
    }
}

impl Pc {
    pub fn sqrt(&self) -> Self {
        Self { value: self.value.sqrt() }
    }
}
