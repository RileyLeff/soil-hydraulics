use num_traits::Float;
use crate::traits::{WaterContentModel, RestrictedParameter};
use crate::errors::{InvalidParam, InvalidSoilModel, UnknowableWaterPotential};

/// Wrapper around arbitrary float type for van genuchten parameter "A"
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alpha<F: Float>(F);

/// Check if VgA is valid
impl<F: Float> RestrictedParameter<F> for Alpha<F> {
    fn is_valid(value: F) -> bool {
        value.is_normal() && value > F::from(0).unwrap()
    }
}

/// VgA
impl<F: Float> Alpha<F> {
    fn new(value: F) -> Result<Self, InvalidParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidParam::BadVgAlpha(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for Alpha<f32>{
    type Error = InvalidParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Alpha::new(value)
    }
}

impl TryFrom<f64> for Alpha<f64>{
    type Error = InvalidParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Alpha::new(value)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct N<F: Float>(F);

impl<F: Float> RestrictedParameter<F> for N<F> {
    fn is_valid(value: F) -> bool {
        value.is_normal() && value > F::from(1.0).unwrap()
    }
}

impl<F: Float> N<F> {
    fn new(value: F) -> Result<Self, InvalidParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidParam::BadVgN(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for N<f32>{
    type Error = InvalidParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        N::new(value)
    }
}

impl TryFrom<f64> for N<f64>{
    type Error = InvalidParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        N::new(value)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Theta<F: Float>(F);

impl<F: Float> RestrictedParameter<F> for Theta<F> {
    fn is_valid(value: F) -> bool {
        value <= F::from(1.0).unwrap() && 
        value >= F::from(0.0).unwrap() && 
        value.is_finite() &&
        !value.is_nan() &&
        !value.is_subnormal()
    }
}

impl<F: Float> Theta<F> {
    fn new(value: F) -> Result<Self, InvalidParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidParam::BadVgTheta(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for Theta<f32>{
    type Error = InvalidParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Theta::new(value)
    }
}

impl TryFrom<f64> for Theta<f64>{
    type Error = InvalidParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Theta::new(value)
    }
}
/// doc
#[derive(Debug)]
pub struct VanGenuchten<F: Float> {
    a: Alpha<F>,
    n: N<F>,
    ts: Theta<F>,
    tr: Theta<F>
}

impl<F: Float> VanGenuchten<F> {

    fn new(a: Alpha<F>, n: N<F>, ts: Theta<F>, tr: Theta<F>) -> Result<Self, InvalidSoilModel<F>> {
        if tr < ts {
            Ok(Self{a, n, ts, tr})
        } else {
            Err(InvalidSoilModel::ThetaDisagreement(tr.get(), ts.get()))
        }
    }
}

impl VanGenuchten<f64> {
    const SAND: Self = Self {
        a: Alpha(1479.5945f64),
        n: N(2.68f64),
        ts: Theta(0.9f64),
        tr: Theta(0.8f64)
    };
}

impl VanGenuchten<f32> {
    const SAND: Self = Self {
        a: Alpha(1479.5945f32),
        n: N(2.68f32),
        ts: Theta(0.9f32),
        tr: Theta(0.8f32)
    };
}

impl<F: Float> WaterContentModel<F> for VanGenuchten<F> {
    
    fn get_water_content(&self, psi: F) -> F {
        if psi <= F::zero() {
            let exponent = (F::one() + (self.a.get() * psi.abs())).powf(-self.n.get());
            self.tr.get() + (self.ts.get() - self.tr.get()) * exponent.powf(F::one() - F::one() / self.n.get())
        } else {
            self.ts.get()
        }
    }

    fn get_water_potential(&self, theta: F) -> Result<F, UnknowableWaterPotential> {
        let m = F::one() - F::one() / self.n.get();
        let base = (theta - self.tr.get()) / (self.ts.get() - self.tr.get());
        let exponent = base.powf(-F::one() / self.n.get());
        self.a.get() * (exponent - F::one()).powf(F::one() / m)
    }

    fn get_effective_saturation(&self, psi: F) -> F {
        (self.get_water_content(psi) - self.tr.get()) / (self.ts.get() - self.tr.get())
    }
}

