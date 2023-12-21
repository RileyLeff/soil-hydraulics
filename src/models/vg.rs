use crate::errors::InvalidVGModelError;

use num_traits::Float;

enum VGPreset{
    Sand,
    LoamySand,
    SandyLoam,
    Silt,
    SiltyLoam,
    SandyClayLoam,
    ClayLoam,
    SiltyClayLoam,
    SiltyClay,
    Clay
}
/// Does this work is this a doc
/// Hello I'm a doc
/// I'm public now i hope i'm visible
pub struct VG {
    a: f64,
    n: f64,
    theta_sat: f64,
    theta_res: f64,
}

impl VG {
    pub fn new(
        a: f64,
        n: f64,
        theta_sat: f64,
        theta_res: f64,
    ) -> Result<Self, InvalidVGModelError> {
        match (
            a <= 0.0 || n <= 0.0,
            theta_sat <= 0.0 || theta_sat >= 1.0,
            theta_res >= theta_sat,
        ) {
            (true, _, _) => Err(InvalidVGModelError::BadNegativeParameter()),
            (_, true, _) => Err(InvalidVGModelError::ParameterOutOfBounds()),
            (_, _, true) => Err(InvalidVGModelError::ThetaDisagreement()),
            _ => Ok(VG {
                a,
                n,
                theta_sat,
                theta_res,
            }),
        }
    }

    pub fn get_preset()

    fn get_water_content(&self, psi: f64) -> f64 {
        let exponent = (1.0 + (self.a * psi.abs())).powf(-self.n);
        self.theta_res + (self.theta_sat - self.theta_res) * exponent.powf(1.0 - 1.0 / self.n)
    }

    fn get_water_potential(&self, theta: f64) -> f64 {
        let m = 1.0 - 1.0 / self.n;
        let base = (theta - self.theta_res) / (self.theta_sat - self.theta_res);
        let exponent = base.powf(-1.0 / self.n);
        self.a * (exponent - 1.0).powf(1.0 / m)
    }
}

impl Default for VG {
    fn default() -> VG {
        VG {
            a: 1479.5945,
            n: 2.68,
            theta_sat: 0.43,
            theta_res: 0.045,
        }
    }
}

// enum BaseSoilModels{
//     Sand,
//     LoamySand,
//     SandyLoam,
//     Silt,
//     SiltyLoam,
//     SandyClayLoam,
//     ClayLoam,
//     SiltyClayLoam,
//     SandyClayLoam,
//     SiltyClay,
//     Clay
// }
