// Decide on an open-source license.

//! Rust implementation of the soil hydraulic model described in [Van Genuchten 1980](doi.org/10.2136/sssaj1980.03615995004400050002x).

#![no_std]
#![feature(error_in_core)]

pub mod errors;
pub mod models;

#[cfg(test)]
mod tests {}
