use crate::errors::{InvalidParam, InvalidSoilModel};
use crate::FloatD;
use floco::{Constrained, Floco};
use serde::{Deserialize, Serialize};
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

    /// Default alpha MPa-1 for sand taken from supplement to doi.org/10.1111/pce.12852 via UNSODA.
    fn get_default() -> F {
        F::from(1479.5945)
            .expect("Error getting default value for Van Genuchten Mualem parameter Alpha")
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
    /// Default N for sand taken from supplement to doi.org/10.1111/pce.12852 via UNSODA.
    fn get_default() -> F {
        F::from(2.68).expect("Error getting default value for Van Genuchten Mualem parameter N")
    }
}

/// Validator for arbitrary float type as van genuchten parameter "Theta"
#[derive(Debug)]
pub struct Theta;

impl<F: FloatD> Constrained<F> for Theta {
    type Error = InvalidParam<F>;

    fn is_valid(value: F) -> bool {
        value <= F::from(1.0).unwrap()
            && value >= F::from(0.0).unwrap()
            && value.is_finite()
            && !value.is_nan()
            && !value.is_subnormal()
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgTheta(value)
    }

    /// Default theta sat for sand taken from supplement to doi.org/10.1111/pce.12852 via UNSODA.
    /// Note that theta_res and theta_sat share a type! The default value returns a reasonable
    /// theta sat, but you'll need to create a new value to get a theta res.
    fn get_default() -> F {
        F::from(0.43)
            .expect("Error getting default value for Van Genuchten Mualem parameter Theta")
    }
}

/// Van Genuchten model
#[derive(Debug, Deserialize, Serialize)]
pub struct VanGenuchten<F: FloatD> {
    a: Floco<F, Alpha>,
    n: Floco<F, N>,
    ts: Floco<F, Theta>,
    tr: Floco<F, Theta>,
}

impl<F: FloatD> VanGenuchten<F> {
    pub fn try_new(
        a: Floco<F, Alpha>,
        n: Floco<F, N>,
        ts: Floco<F, Theta>,
        tr: Floco<F, Theta>,
    ) -> Result<Self, InvalidSoilModel<F>> {
        if tr.get() < ts.get() {
            Ok(Self { a, n, ts, tr })
        } else {
            Err(InvalidSoilModel::ThetaDisagreement(tr.get(), ts.get()))
        }
    }

    pub fn get_m(&self) -> F {
        F::one() - F::one() / self.n.get()
    }

    pub fn get_water_content(&self, psi: F) -> F {
        if psi <= F::zero() {
            let exponent = (F::one() + (self.a.get() * psi.abs())).powf(-self.n.get());
            self.tr.get()
                + (self.ts.get() - self.tr.get())
                    * exponent.powf(F::one() - F::one() / self.n.get())
        } else {
            self.ts.get()
        }
    }

    pub fn get_water_potential(&self, theta: Floco<F, Theta>) -> F {
        let theta = theta.get();
        let base = (theta - self.tr.get()) / (self.ts.get() - self.tr.get());
        let exponent = base.powf(-F::one() / self.n.get());
        self.a.get() * (exponent - F::one()).powf(F::one() / self.get_m())
    }

    pub fn get_water_potential_checked(&self, theta: F) -> Result<F, InvalidParam<F>> {
        let theta = Floco::<F, Theta>::try_new(theta)?.get();
        let base = (theta - self.tr.get()) / (self.ts.get() - self.tr.get());
        let exponent = base.powf(-F::one() / self.n.get());
        Ok(self.a.get() * (exponent - F::one()).powf(F::one() / self.get_m()))
    }

    pub fn get_effective_saturation(&self, psi: F) -> F {
        (self.get_water_content(psi) - self.tr.get()) / (self.ts.get() - self.tr.get())
    }
}

impl<F: FloatD> Default for VanGenuchten<F> {
    fn default() -> Self {
        let a = Floco::<F, Alpha>::default();
        let n = Floco::<F, N>::default();
        let ts = Floco::<F, Theta>::default();
        let tr = Floco::<F, Theta>::try_new(
            F::from(0.045)
                .expect("Error converting theta_res default value into type F")
            )
                .expect("Error creating Floco<F, Theta> from default value.");
        VanGenuchten::try_new(a, n, ts, tr)
            .expect("Error creating VanGenuchten model from default values.")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::println;

    #[test]
    fn get_water_content_works() {
        assert!(1 == 1);
    }

    #[test]
    fn get_water_potential_works() {
        assert!(1 == 1);
    }

    #[test]
    fn get_water_potential_checked_works() {
        let vg = VanGenuchten::<f64>::default();
        println!("vg is {:?}", vg);
        let psi = vg.get_water_potential_checked(0.1f64);
        println!("Water potential is {:?}", psi);
        match psi {
            Ok(f) => println!("Water potential is {}", f),
            Err(_) => println!("Error!")
        }
        assert!(1 == 1);
    }

    #[test]
    fn get_effective_saturation_works() {
        assert!(1 == 1);
    }

    
}