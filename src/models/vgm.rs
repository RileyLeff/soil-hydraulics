// use crate::errors::InvalidVGModelError;
// use crate::models::vg::VG;

// use num_traits::Float;

// /// Van Genuchten - Mualem model for soil moisture and hydraulic conductivity.
// pub struct VGM<F: Float> {
//     vg: VG<F>,
//     k_sat: f64,
// }

// impl<F: Float> VGM<F> {
//     pub fn new(
//         vg: VG<F>,
//         k_sat: F,
//     ) -> Result<Self, InvalidVGModelError> {
//         if F::is_sign_negative(k_sat) {
//             Err(InvalidVGModelError::BadNegativeParameter())
//         } else {
//             Ok(VGM{vg, k_sat})
//         }
//     }

//     fn get_water_content(&self, psi: F) -> F {
//         let exponent = (1.0 + (self.a * psi.abs())).powf(-self.n);
//         self.theta_res + (self.theta_sat - self.theta_res) * exponent.powf(1.0 - 1.0 / self.n)
//     }

//     fn get_water_potential(&self, theta: f64) -> f64 {
//         let m = 1.0 - 1.0 / self.n;
//         let base = (theta - self.theta_res) / (self.theta_sat - self.theta_res);
//         let exponent = base.powf(-1.0 / self.n);
//         self.a * (exponent - 1.0).powf(1.0 / m)
//     }

//     fn get_hydraulic_conductivity(&self) -> f64 {
//         69
//     }
// }

// impl<F: Float> Default for VGM<F> {
//     fn default() -> VGM<F> {
//         VGM {
//             vg: crate::models::vg::default(),
//             k_sat: 29.7
//         }
//     }
// }
