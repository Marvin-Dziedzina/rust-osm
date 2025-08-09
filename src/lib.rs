#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
// #![deny(missing_docs)] // TODO: Enable before shipping
#![deny(unreachable_pub)]
#![deny(unreachable_code, dead_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![warn(nonstandard_style)]
#![warn(future_incompatible)]

//! A OSM API wrapper.

// Ensure that one of `coordinate_f32` or `coordinate_f64` is enabled.
#[cfg(not(any(feature = "coordinate_f32", feature = "coordinate_f64")))]
compile_error!("One of `coordinate_f32` or `coordinate_f64` must be enabled.");

// Ensure that `coordinate_f32` and `coordinate_f64` are never enabled together.
#[cfg(all(feature = "coordinate_f32", feature = "coordinate_f64"))]
compile_error!("Features `coordinate_f32` and `coordinate_f64` can not be enabled together.");

pub mod coord;
pub mod rest_methods;

#[cfg(feature = "overpass")]
pub mod overpass;
