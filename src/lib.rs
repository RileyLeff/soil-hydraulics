//! Rust implementation of the soil hydraulic model described in
//! [Van Genuchten 1980](doi.org/10.2136/sssaj1980.03615995004400050002x).
//! 

// configure no_std if both std_math and std_errors features are inactive
#![cfg_attr(all(not(feature = "std_math"), not(feature = "std_errors")), no_std)]
// activate error_in_core feature if std_errors feature inactive
#![cfg_attr(all(not(feature = "std_errors"),), feature(error_in_core))]

use num_traits::Float;

// ensure at least one of libm and std_math are enabled
#[cfg(all(not(feature = "libm"), not(feature = "std_math")))]
compile_error!("One of the 'libm' or 'std_math' features must be enabled.");

// ensure libm and std_math can't be turned on concurrently
#[cfg(all(feature = "libm", feature = "std_math"))]
compile_error!(
    "The 'libm' (enabled by default) and 'std_math' features cannot be enabled simultaneously."
);

#[cfg(feature = "libm")]
use core::fmt::Display;
#[cfg(feature = "libm")]
use core::fmt::Debug;

#[cfg(feature = "std_math")]
use std::fmt::Display;
#[cfg(feature = "std_math")]
use std::fmt::Debug;

#[cfg(test)]
extern crate std;

pub trait FloatD: Display + Debug + Float {}
impl<T: Display + Debug + Float> FloatD for T {}
// type FloatD = F where F: Display + Float;

pub mod errors;
pub mod models;

#[cfg(test)]
mod tests {}
