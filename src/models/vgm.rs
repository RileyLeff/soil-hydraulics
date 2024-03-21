use crate::errors::InvalidParam;
use crate::models::vg::VanGenuchten;
use crate::FloatD;
use floco::{Constrained, Floco};
use serde::{Deserialize, Serialize};

/// Validator for arbitrary float type as van genuchten - mualem parameter "KSat"
#[derive(Debug)]
pub struct KSat;

impl<F: FloatD> Constrained<F> for KSat {
    type Error = InvalidParam<F>;

    fn is_valid(value: F) -> bool {
        value >= F::from(0).unwrap()
            && value.is_finite()
            && !value.is_subnormal()
            && !value.is_nan()
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgMKSat(value)
    }

    fn get_default() -> F {
        F::from(0.5).expect("Error getting default value for Van Genuchten Mualem parameter KSat")
    }
}

/// Typically assumed to be 0.5, i.e. taking square root of other terms.
/// See doi.org/10.2136/vzj2005.0005 for more in-depth discussion.
#[derive(Debug)]
pub struct L;

impl<F: FloatD> Constrained<F> for L {
    type Error = InvalidParam<F>;

    fn is_valid(_value: F) -> bool {
        true // perhaps restrict in future
    }

    fn emit_error(value: F) -> Self::Error {
        Self::Error::BadVgMKSat(value)
    }

    fn get_default() -> F {
        F::from(0.5).expect("Error getting default value for Van Genuchten Mualem parameter L")
    }
}

/// Van Genuchten - Mualem model for soil moisture and hydraulic conductivity.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VanGenuchtenMualem<F: FloatD> {
    vg: VanGenuchten<F>,
    ksat: Floco<F, KSat>,
    l: Floco<F, L>,
}

impl<F: FloatD> VanGenuchtenMualem<F> {
    pub fn new(vg: VanGenuchten<F>, ksat: Floco<F, KSat>, l: Floco<F, L>) -> Self {
        Self { vg, ksat, l }
    }

    #[allow(dead_code)]
    fn get_hydraulic_conductivity(&self, psi: F) -> F {
        if psi > F::zero() {
            self.ksat.get()
        } else {
            let se = self.vg.get_effective_saturation(psi);
            let m = self.vg.get_m();
            let first_term = self.ksat.get() * se.powf(self.l.get());
            let second_term =
                (F::one() - (F::one() - se.powf(F::one() / m)).powf(m)).powf(F::one() + F::one());
            first_term * second_term
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::println;

    #[test]
    fn get_hydraulic_conductivity_works() {
        let vgm = VanGenuchtenMualem::<f64>::default();
        let cool = vgm.get_hydraulic_conductivity(-1.5f64);
        println!("cool is {:?}", cool);
        assert!(1 == 1);
    }
}
