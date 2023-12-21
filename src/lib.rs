// Decide on an open-source license.

//! Rust implementation of the soil hydraulic model described in 
//! [Van Genuchten 1980](doi.org/10.2136/sssaj1980.03615995004400050002x).

#![cfg_attr(
    all(
        not(feature = "std_math"), 
        not(feature = "std_errors")
    ), no_std)
]

#![cfg_attr(
    all(not(feature = "std_errors"), 
    ), feature(error_in_core)
)]

#[cfg(all(not(feature = "libm"), not(feature = "std_math")))]
compile_error!(
    "One of the 'libm' or 'std_math' features must be enabled."
);

#[cfg(all(feature = "libm", feature = "std_math"))]
compile_error!(
    "The 'libm' (enabled by default) and 'std_math' features cannot be enabled simultaneously."
);



pub mod errors;
pub mod models;

#[cfg(test)]
mod tests {}
