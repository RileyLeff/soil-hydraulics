use num_traits::Float;

trait WaterContentModel {
    fn get_water_content(&self, psi: f64) -> f64;
    fn get_water_potential(&self, theta: f64) -> f64;
}

pub trait RestrictedParameter<F: Float> {
    fn is_valid(value: F) -> bool;
}