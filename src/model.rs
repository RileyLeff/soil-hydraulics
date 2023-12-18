use libm::{fabs, pow};
use crate::errors::InvalidModelError;

#[derive(Debug, PartialEq)]
struct VanGenuchtenModel {
    a: f64, 
    n: f64,
    k_sat: f64,
    k_max: f64,
    theta_sat: f64, 
    theta_res: f64
}

impl VanGenuchtenModel {
    fn new(a: f64, n: f64, k_sat: f64, k_max: f64, theta_sat: f64, theta_res: f64) -> Result<Self, InvalidModelError> {
        if a <= 0.0 {
            Err(InvalidModelError::BadAlpha())
        } else if n <= 0.0 {
            Err(InvalidModelError::BadN())
        } else if k_sat <= 0.0 {
            Err(InvalidModelError::BadKSat())
        } else if k_max <= 0.0 {
            Err(InvalidModelError::BadKMax())
        } else if theta_sat <= 0.0 || theta_sat >= 1.0 {
            Err(InvalidModelError::BadThetaSat())
        } else if theta_res <= 0.0 || theta_res >= 1.0 {
            Err(InvalidModelError::BadThetaRes())
        } else if theta_res >= theta_sat {
            Err(InvalidModelError::ThetaSatResDisagreement())
        } else {
            Ok(VanGenuchtenModel{a, n, k_sat, k_max, theta_sat, theta_res})
        }
    }

    fn get_water_content(&self, psi: f64) -> f64 {
        self.theta_res + 
        (self.theta_sat - self.theta_res) / 
        pow(
            pow(
                1.0 + (self.a * fabs(psi)),
                self.n
            ), 
            1.0 - self.n
        )
    }

    fn get_water_potential(&self) -> f64{
        -1.0
    }

    fn get_hydraulic_conductivity(&self) -> f64 {
        self.k_max
    }
}

impl Default for VanGenuchtenModel {
    fn default() -> VanGenuchtenModel {
        VanGenuchtenModel {a: 1479.5945, n: 2.68, k_sat: 29.7, k_max: 30305.88, theta_sat: 0.43, theta_res: 0.045}
    }
}
