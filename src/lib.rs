// #![deny(missing_debug_implementations)]
// #![deny(missing_docs)]
// #![deny(unreachable_pub)]
// #![warn(rust_2018_idioms)]

//! A OSM API wrapper.

pub mod coordinates;
pub mod rest_methods;

#[cfg(feature = "overpass")]
pub mod overpass;
