use crate::unit::mass::earth::EarthMass;

pub mod earth;

pub trait Massed {
    fn raw_value(&self) -> f64;
}

#[derive(Clone)]
pub enum Mass {
    Earth(EarthMass),
}

impl Massed for Mass {
    fn raw_value(&self) -> f64 {
        match self {
            Self::Earth(v) => v.raw_value(),
        }
    }
}

impl From<EarthMass> for Mass {
    fn from(value: EarthMass) -> Self {
        Mass::Earth(value)
    }
}

impl From<&EarthMass> for Mass {
    fn from(value: &EarthMass) -> Self {
        Mass::from(value.clone())
    }
}
