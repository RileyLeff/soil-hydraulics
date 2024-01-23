use num_traits::Float;

use crate::errors::UnknowableWaterPotential;

pub trait WaterContentModel<F: Float> {
    fn get_water_content(&self, psi: F) -> F;
    fn get_water_potential(&self, theta: F) -> Result<F, UnknowableWaterPotential>;
    fn get_effective_saturation(&self, psi: F) -> F;
}

pub trait HydraulicConductivityModel<F: Float> {
    fn get_conductivity_from_water_potential(&self, psi: F) -> F;
    fn get_conductivity_from_water_content(&self, psi: F) -> F;
}

pub trait RestrictedParameter<F: Float> {
    fn is_valid(value: F) -> bool;
}