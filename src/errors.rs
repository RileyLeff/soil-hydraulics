use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidVGModelError {
    #[error("Parameters a, n, k_max, and k_sat must be positive")]
    BadNegativeParameter(),
    #[error("Parameters theta_sat and theta_res must be between zero and one")]
    ParameterOutOfBounds(),
    #[error("Parameter theta_sat must be greater than parameter theta_res")]
    ThetaDisagreement(),
}