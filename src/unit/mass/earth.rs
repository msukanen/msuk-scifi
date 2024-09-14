use super::Massed;

#[derive(Clone)]
pub struct EarthMass {
    value: f64
}

impl Massed for EarthMass {
    fn raw_value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for EarthMass {
    fn from(value: f64) -> Self {
        Self { value }
    }
}
