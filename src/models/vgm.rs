use crate::errors::InvalidParam;
use crate::models::vg::VanGenuchten;
use crate::traits::{RestrictedParameter, WaterContentModel};

use num_traits::Float;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KSat<F: Float>(F);


impl<F: Float> RestrictedParameter<F> for KSat<F> {
    fn is_valid(value: F) -> bool {
        value >= F::from(0).unwrap() &&
        value.is_finite() &&
        !value.is_subnormal() &&
        !value.is_nan()
    }
}


impl<F: Float> KSat<F> {
    fn new(value: F) -> Result<Self, InvalidParam<F>> {
        if Self::is_valid(value) {
            Ok(Self(value))
        } else {
            Err(InvalidParam::BadVgMKSat(value))
        }
    }

    fn get(&self) -> F {
        self.0
    }
}

impl TryFrom<f32> for KSat<f32>{
    type Error = InvalidParam<f32>;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        KSat::new(value)
    }
}

impl TryFrom<f64> for KSat<f64>{
    type Error = InvalidParam<f64>;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        KSat::new(value)
    }
}


/// Van Genuchten - Mualem model for soil moisture and hydraulic conductivity.
pub struct VanGenuchtenMualem<F: Float> {
    vg: VanGenuchten<F>,
    ks: KSat<F>,
}

impl<F: Float> VanGenuchtenMualem<F> {

    pub fn new(vg: VanGenuchten<F>, ks: KSat<F>) -> Self {
        Self{vg, ks}

    }

    fn get_water_content(&self, psi: F) -> F {
        let exponent = (1.0 + (self.a * psi.abs())).powf(-self.n);
        self.theta_res + (self.theta_sat - self.theta_res) * exponent.powf(1.0 - 1.0 / self.n)
    }

    fn get_water_potential(&self, theta: f64) -> f64 {
        let m = 1.0 - 1.0 / self.n;
        let base = (theta - self.theta_res) / (self.theta_sat - self.theta_res);
        let exponent = base.powf(-1.0 / self.n);
        self.a * (exponent - 1.0).powf(1.0 / m)
    }

    fn get_hydraulic_conductivity(&self) -> f64 {
        69
    }
}

impl<F: Float> WaterContentModel<F> for VanGenuchtenMualem<F> {
    
    fn get_water_content(&self, psi: F) -> F {
        self.vg.get_water_content(psi)
    }

    fn get_water_potential(&self, theta: F) -> F {
        self.vg.get_water_potential(theta)
    }
    
}

// impl<F: Float> Default for VanGenuchtenMualem<F> {
//     fn default() -> VanGenuchtenMualem<F> {
//         VanGenuchtenMualem {
//             vg: crate::models::vg::default(),
//             k_sat: 29.7
//         }
//     }
// }
