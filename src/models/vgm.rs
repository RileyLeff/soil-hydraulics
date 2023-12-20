use crate::errors::InvalidVGModelError;

#[cfg(not(feature = "std"))]
use libm::{fabs, pow};

/// Does this work is this a doc
/// Hello I'm a doc
/// I'm public now i hope i'm visible
pub struct VanGenuchtenModel {
    a: f64,
    n: f64,
    k_sat: f64,
    k_max: f64,
    theta_sat: f64,
    theta_res: f64,
}

impl VanGenuchtenModel {
    pub fn new(
        a: f64,
        n: f64,
        k_sat: f64,
        k_max: f64,
        theta_sat: f64,
        theta_res: f64,
    ) -> Result<Self, InvalidVGModelError> {
        match (
            a <= 0.0 || n <= 0.0 || k_sat <= 0.0 || k_max <= 0.0,
            theta_sat <= 0.0 || theta_sat >= 1.0,
            theta_res >= theta_sat,
        ) {
            (true, _, _) => Err(InvalidVGModelError::BadNegativeParameter()),
            (_, true, _) => Err(InvalidVGModelError::ParameterOutOfBounds()),
            (_, _, true) => Err(InvalidVGModelError::ThetaDisagreement()),
            _ => Ok(VanGenuchtenModel {
                a,
                n,
                k_sat,
                k_max,
                theta_sat,
                theta_res,
            }),
        }
    }

    fn get_water_content(&self, psi: f64) -> f64 {
        let exponent = pow(1.0 + (self.a * fabs(psi)), -self.n);
        self.theta_res + (self.theta_sat - self.theta_res) * pow(exponent, 1.0 - 1.0 / self.n)
    }

    fn get_water_potential(&self, theta: f64) -> f64 {
        let m = 1.0 - 1.0 / self.n;
        let base = (theta - self.theta_res) / (self.theta_sat - self.theta_res);
        let exponent = pow(base, -1.0 / self.n);
        self.a * pow(exponent - 1.0, 1.0 / m)
    }

    fn get_hydraulic_conductivity(&self) -> f64 {
        self.k_max
    }
}

impl Default for VanGenuchtenModel {
    fn default() -> VanGenuchtenModel {
        VanGenuchtenModel {
            a: 1479.5945,
            n: 2.68,
            k_sat: 29.7,
            k_max: 30305.88,
            theta_sat: 0.43,
            theta_res: 0.045,
        }
    }
}
