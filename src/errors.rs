use num_traits::Float;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidVGParam<F: Float> {
    #[error("Parameter 'a' (alpha) must be greater than zero. It is currently set to {0}.")]
    BadVgAlpha(F),
    #[error("Parameter 'n' must be greater than one. It is currently set to {0}.")]
    BadVgN(F),
    #[error("Parameters 'tr' and 'ts' must be greater than zero, less than or equal to one, and not a subnormal value.  The offending parameter is currently set to {0}.")]
    BadVgTheta(F),
    #[error("Parameter 'theta_sat' must be greater than 'theta_res'. They are currently set to {0} and {1}, respectively.")]
    BadVgThetaDisagreement(F, F)
}
