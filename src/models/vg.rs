use num_traits::Float;
use crate::traits::RestrictedParameter;
use crate::errors::InvalidVGParam;

/// Wrapper around arbitrary float type for van genuchten parameter "A"
#[derive(Debug)]
pub struct VgA<F: Float>(F);

/// Check if VgA is valid
impl<F: Float> RestrictedParameter<F> for VgA<F> {
    fn is_valid(value: F) -> bool {
        value.is_normal()
    }
}

/// VgA
impl<F: Float> VgA<F> {
    fn new(value: F) -> Result<Self, InvalidVGParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidVGParam::BadVgAlpha(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for VgA<f32>{
    type Error = InvalidVGParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        VgA::new(value)
    }
}

impl TryFrom<f64> for VgA<f64>{
    type Error = InvalidVGParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        VgA::new(value)
    }
}

#[derive(Debug)]
pub struct VgN<F: Float>(F);

impl<F: Float> RestrictedParameter<F> for VgN<F> {
    fn is_valid(value: F) -> bool {
        value.is_normal() && value > F::from(1.0).unwrap()
    }
}

impl<F: Float> VgN<F> {
    fn new(value: F) -> Result<Self, InvalidVGParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidVGParam::BadVgN(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for VgN<f32>{
    type Error = InvalidVGParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        VgN::new(value)
    }
}

impl TryFrom<f64> for VgN<f64>{
    type Error = InvalidVGParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        VgN::new(value)
    }
}

#[derive(Debug)]
pub struct VgT<F: Float>(F);

impl<F: Float> RestrictedParameter<F> for VgT<F> {
    fn is_valid(value: F) -> bool {
        value <= F::from(1.0).unwrap() && value >= F::from(0.0).unwrap() && !value.is_subnormal()
    }
}

impl<F: Float> VgT<F> {
    fn new(value: F) -> Result<Self, InvalidVGParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidVGParam::BadVgTheta(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for VgT<f32>{
    type Error = InvalidVGParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        VgT::new(value)
    }
}

impl TryFrom<f64> for VgT<f64>{
    type Error = InvalidVGParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        VgT::new(value)
    }
}
/// doc
#[derive(Debug)]
pub struct Vg<F: Float> {
    a: VgA<F>,
    n: VgN<F>,
    ts: VgT<F>,
    tr: VgT<F>
}

impl<F: Float> Vg<F> {

    fn get_water_content(&self, psi: F) -> F {
        let exponent = (F::one() + (self.a.get() * psi.abs())).powf(-self.n.get());
        self.tr.get() + (self.ts.get() - self.tr.get()) * exponent.powf(F::one() - F::one() / self.n.get())
    }

    fn get_water_potential(&self, theta: F) -> F {
        let m = F::one() - F::one() / self.n.get();
        let base = (theta - self.tr.get()) / (self.ts.get() - self.tr.get());
        let exponent = base.powf(-F::one() / self.n.get());
        self.a.get() * (exponent - F::one()).powf(F::one() / m)
    }
}

impl Vg<f64> {
    const SAND: Self = Self {
        a: VgA(1479.5945f64),
        n: VgN(2.68f64),
        ts: VgT(0.9f64),
        tr: VgT(0.8f64)
    };
}

impl Vg<f32> {
    const SAND: Self = Self {
        a: VgA(1479.5945f32),
        n: VgN(2.68f32),
        ts: VgT(0.9f32),
        tr: VgT(0.8f32)
    };
}