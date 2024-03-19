use floco::{Constrained, Floco};
use serde::{Deserialize, Serialize};
use crate::errors::{InvalidParam, InvalidSoilModel};
use crate::FloatD;

/// Validator for arbitrary float type as van genuchten parameter "A"
#[derive(Debug)]
pub struct Alpha;

impl<F: FloatD> Constrained<F> for Alpha {
    type Error = InvalidParam<F>;

    fn is_valid(value: F) -> bool {
        value.is_normal() && value > F::from(0).unwrap()
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgAlpha(value)
    }

    fn get_default() -> F {
        F::from(0.5f64).expect("Error getting default value for Van Genuchten Mualem parameter Alpha")
    }

}

/// Validator for arbitrary float type as van genuchten parameter "N"
#[derive(Debug)]
pub struct N;

impl<F: FloatD> Constrained<F> for N {
    type Error = InvalidParam<F>;

    fn is_valid(value: F) -> bool {
        value.is_normal() && value > F::from(1.0).unwrap()
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgN(value)
    }

    fn get_default() -> F {
        F::from(0.5f64).expect("Error getting default value for Van Genuchten Mualem parameter N")
    }
}

/// Validator for arbitrary float type as van genuchten parameter "Theta"
#[derive(Debug)]
pub struct Theta;

impl<F: FloatD> Constrained<F> for Theta {
    type Error = InvalidParam<F>;

    fn is_valid(value: F) -> bool {
        value <= F::from(1.0).unwrap() && 
        value >= F::from(0.0).unwrap() && 
        value.is_finite() &&
        !value.is_nan() &&
        !value.is_subnormal()
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgTheta(value)
    }

    fn get_default() -> F {
        F::from(0.5f64).expect("Error getting default value for Van Genuchten Mualem parameter Theta")
    }
}

/// Van Genuchten model
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VanGenuchten<F: FloatD> {
    a: Floco<F, Alpha>,
    n: Floco<F, N>,
    ts: Floco<F, Theta>,
    tr: Floco<F, Theta>
}

impl<F: FloatD> VanGenuchten<F> {
    pub fn try_new(
        a: Floco<F, Alpha>, 
        n: Floco<F, N>, 
        ts: Floco<F, Theta>, 
        tr: Floco<F, Theta>
    ) -> Result<Self, InvalidSoilModel<F>> {
        if tr.get() < ts.get() {
            Ok(Self{a, n, ts, tr})
        } else {
            Err(InvalidSoilModel::ThetaDisagreement(tr.get(), ts.get()))
        }
    }

    pub fn get_m(&self) -> F {
        return F::one() - F::one() / self.n.get();
    }

    pub fn get_water_content(&self, psi: F) -> F {
        if psi <= F::zero() {
            let exponent = (F::one() + (self.a.get() * psi.abs())).powf(-self.n.get());
            self.tr.get() + (self.ts.get() - self.tr.get()) * exponent.powf(F::one() - F::one() / self.n.get())
        } else {
            self.ts.get()
        }
    }

    pub fn get_water_potential(&self, theta: F) -> F {
        let base = (theta - self.tr.get()) / (self.ts.get() - self.tr.get());
        let exponent = base.powf(-F::one() / self.n.get());
        self.a.get() * (exponent - F::one()).powf(F::one() / self.get_m())
    }

    pub fn get_effective_saturation(&self, psi: F) -> F {
        (self.get_water_content(psi) - self.tr.get()) / (self.ts.get() - self.tr.get())
    }
}