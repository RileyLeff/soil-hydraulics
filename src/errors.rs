use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InvalidModelError {
    #[error("Parameter alpha must be positive")]
    BadAlpha(),
    #[error("Parameter n must be positive")]
    BadN(),
    #[error("Parameter k_s must be positive")]
    BadKSat(),
    #[error("Parameter k_max must be positive")]
    BadKMax(),
    #[error("Parameter theta_s must be between 0 and 1")]
    BadThetaSat(),
    #[error("Parameter theta_r must be between 0 and 1")]
    BadThetaRes(),
    #[error("Parameter theta_s must be greater than parameter theta_r")]
    ThetaSatResDisagreement(),
}