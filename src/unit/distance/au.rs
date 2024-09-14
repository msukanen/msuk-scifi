use std::ops::{Add, Div, Mul, Sub};

use super::{km::Km, ly::Ly, mi::Mi, pc::Pc, Distance, Distanced};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Au {
    value: f64,
}

impl Distanced for Au {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Au {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl From<Km> for Au {
    fn from(value: Km) -> Self {
        Self { value: value.raw_value() / 149_597_871.0 }
    }
}

impl From<Mi> for Au {
    fn from(value: Mi) -> Self {
        Self::from(Km::from(value))
    }
}

impl From<Ly> for Au {
    fn from(value: Ly) -> Self {
        Self { value: value.raw_value() * 63241.0771 }
    }
}

impl From<Pc> for Au {
    fn from(value: Pc) -> Self {
        Self::from(Ly::from(value))
    }
}

impl From<Distance> for Au {
    fn from(value: Distance) -> Self {
        match value {
            Distance::Km(a) => Au::from(a),
            Distance::Mi(a) => Au::from(a),
            Distance::Au(a) => a,
            Distance::Ly(a) => Au::from(a),
            Distance::Pc(a) => Au::from(a)
        }
    }
}

impl std::fmt::Display for Au {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AU", self.value)
    }
}

impl Mul<f64> for Au {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self { value: self.value * rhs }
    }
}

impl Add<Au> for f64 {
    type Output = Au;
    fn add(self, rhs: Au) -> Self::Output {
        rhs + self
    }
}

impl Add<f64> for Au {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self { value: self.value + rhs }
    }
}

impl Sub<Au> for f64 {
    type Output = Au;
    fn sub(self, rhs: Au) -> Self::Output {
        Au { value: self - rhs.value }
    }
}

impl Sub<f64> for Au {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        Self { value: self.value - rhs}
    }
}

impl Div<f64> for Au {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self { value: self.value / rhs }
    }
}

impl Div<Au> for f64 {
    type Output = Au;
    fn div(self, rhs: Au) -> Self::Output {
        Au::from( self / rhs.value )
    }
}

impl Au {
    pub fn sqrt(&self) -> Self {
        Self { value: self.value.sqrt() }
    }
}
