use std::ops::{Add, Div, Mul, Sub};

pub struct K {
    value: f64,
}

impl K {
    fn validate(value: f64) -> f64 {
        if value < 0.0 {
            0.0
        } else {
            value
        }
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

    pub fn value(&self) -> f64 {
        self.as_f64()
    }
}

impl std::fmt::Display for K {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}K", self.value)
    }
}

impl Add<f64> for K {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        K { value: Self::validate(self.value + rhs) }
    }
}

impl Add<K> for K {
    type Output = Self;
    fn add(self, rhs: K) -> Self::Output {
        self + rhs.value
    }
}

impl Sub<f64> for K {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        K { value: Self::validate(self.value - rhs) }
    }
}

impl Sub<K> for K {
    type Output = Self;
    fn sub(self, rhs: K) -> Self::Output {
        self - rhs.value
    }
}

impl From<f64> for K {
    fn from(value: f64) -> Self {
        K { value: Self::validate(value) }
    }
}

impl From<i32> for K {
    fn from(value: i32) -> Self {
        K { value: Self::validate(value as f64) }
    }
}

impl Mul<i32> for K {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        K { value: Self::validate(self.value * rhs as f64) }
    }
}

impl Mul<i64> for K {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        K { value: Self::validate(self.value * rhs as f64) }
    }
}

impl Mul<f64> for K {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        K { value: Self::validate(self.value * rhs) }
    }
}

impl Mul<K> for f64 {
    type Output = K;
    fn mul(self, rhs: K) -> Self::Output {
        rhs * self
    }
}

impl Mul<K> for K {
    type Output = Self;
    fn mul(self, rhs: K) -> Self::Output {
        self * rhs.value
    }
}

impl Div<f64> for K {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        K { value: self.value / rhs }
    }
}

impl Div<K> for f64 {
    type Output = K;
    fn div(self, rhs: K) -> Self::Output {
        K { value: K::validate(self / rhs.value) }
    }
}
