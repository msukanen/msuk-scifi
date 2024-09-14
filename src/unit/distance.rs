use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Sub};

use crate::unit::distance::{km::Km, au::Au, ly::Ly, mi::Mi, pc::Pc};

pub mod au;
pub mod km;
pub mod ly;
pub mod mi;
pub mod pc;

pub trait Distanced {
    fn raw_value(&self) -> f64;
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Distance {
    Km(Km),
    Mi(Mi),
    Au(Au),
    Ly(Ly),
    Pc(Pc),
}

impl Distanced for Distance {
    fn raw_value(&self) -> f64 {
        match self {
            Self::Km(a) => a.raw_value(),
            Self::Mi(a) => a.raw_value(),
            Self::Au(a) => a.raw_value(),
            Self::Ly(a) => a.raw_value(),
            Self::Pc(a) => a.raw_value()
        }
    }
}

impl From<Km> for Distance {
    fn from(value: Km) -> Self {
        Distance::Km(value)
    }
}

impl From<Mi> for Distance {
    fn from(value: Mi) -> Self {
        Distance::Mi(value)
    }
}

impl From<Au> for Distance {
    fn from(value: Au) -> Self {
        Distance::Au(value)
    }
}

impl From<Ly> for Distance {
    fn from(value: Ly) -> Self {
        Distance::Ly(value)
    }
}

impl From<Pc> for Distance {
    fn from(value: Pc) -> Self {
        Distance::Pc(value)
    }
}

impl std::fmt::Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Km(x) => format!("{x}"),
            Self::Mi(x) => format!("{x}"),
            Self::Au(x) => format!("{x}"),
            Self::Ly(x) => format!("{x}"),
            Self::Pc(x) => format!("{x}"),
        })
    }
}

impl Mul<Distance> for f64 {
    type Output = Distance;
    fn mul(self, rhs: Distance) -> Self::Output {
        rhs * self
    }
}

impl Mul<f64> for Distance {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Distance::Km(a) => Distance::Km(a * rhs),
            Distance::Mi(a) => Distance::Mi(a * rhs),
            Distance::Au(a) => Distance::Au(a * rhs),
            Distance::Ly(a) => Distance::Ly(a * rhs),
            Distance::Pc(a) => Distance::Pc(a * rhs),
        }
    }
}

impl Add<Distance> for f64 {
    type Output = Distance;
    fn add(self, rhs: Distance) -> Self::Output {
        rhs + self
    }
}

impl Add<f64> for Distance {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Self::Km(a) => Self::Km(a + rhs),
            Self::Mi(a) => Self::Mi(a + rhs),
            Self::Au(a) => Self::Au(a + rhs),
            Self::Ly(a) => Self::Ly(a + rhs),
            Self::Pc(a) => Self::Pc(a + rhs)
        }
    }
}

impl Sub<Distance> for f64 {
    type Output = Distance;
    fn sub(self, rhs: Distance) -> Self::Output {
        match rhs {
            Distance::Km(a) => Distance::Km(self - a),
            Distance::Mi(a) => Distance::Mi(self - a),
            Distance::Au(a) => Distance::Au(self - a),
            Distance::Ly(a) => Distance::Ly(self - a),
            Distance::Pc(a) => Distance::Pc(self - a)
        }
    }
}

impl Sub<f64> for Distance {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            Self::Km(a) => Self::Km(a + rhs),
            Self::Mi(a) => Self::Mi(a + rhs),
            Self::Au(a) => Self::Au(a + rhs),
            Self::Ly(a) => Self::Ly(a + rhs),
            Self::Pc(a) => Self::Pc(a + rhs),
        }
    }
}

impl Div<f64> for Distance {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Self::Km(a) => Self::Km(a / rhs),
            Self::Mi(a) => Self::Mi(a / rhs),
            Self::Au(a) => Self::Au(a / rhs),
            Self::Ly(a) => Self::Ly(a / rhs),
            Self::Pc(a) => Self::Pc(a / rhs),
        }
    }
}

impl Div<Distance> for f64 {
    type Output = Distance;
    fn div(self, rhs: Distance) -> Self::Output {
        match rhs {
            Distance::Km(a) => Distance::Km(Km::from(self / a)),
            Distance::Mi(a) => Distance::Mi(Mi::from(self / a)),
            Distance::Au(a) => Distance::Au(Au::from(self / a)),
            Distance::Ly(a) => Distance::Ly(Ly::from(self / a)),
            Distance::Pc(a) => Distance::Pc(Pc::from(self / a)),
        }
    }
}

impl DivAssign<f64> for Distance {
    fn div_assign(&mut self, rhs: f64) {
        match self {
            Self::Km(a) => *a = Km::from((*a) / rhs),
            Self::Mi(a) => *a = Mi::from((*a) / rhs),
            Self::Au(a) => *a = Au::from((*a) / rhs),
            Self::Ly(a) => *a = Ly::from((*a) / rhs),
            Self::Pc(a) => *a = Pc::from((*a) / rhs),
        }
    }
}

impl MulAssign<f64> for Distance {
    fn mul_assign(&mut self, rhs: f64) {
        match self {
            Self::Km(a) => *a = Km::from((*a) * rhs),
            Self::Mi(a) => *a = Mi::from((*a) * rhs),
            Self::Au(a) => *a = Au::from((*a) * rhs),
            Self::Ly(a) => *a = Ly::from((*a) * rhs),
            Self::Pc(a) => *a = Pc::from((*a) * rhs),
        }
    }
}

impl Distance {
    pub fn sqrt(&self) -> Self {
        match self {
            Self::Km(a) => Self::Km(a.sqrt()),
            Self::Mi(a) => Self::Mi(a.sqrt()),
            Self::Au(a) => Self::Au(a.sqrt()),
            Self::Ly(a) => Self::Ly(a.sqrt()),
            Self::Pc(a) => Self::Pc(a.sqrt()),
        }
    }
}
