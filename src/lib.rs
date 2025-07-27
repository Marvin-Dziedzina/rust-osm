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
#![warn(rustdoc::missing_doc_code_examples)]

//! A OSM API wrapper.

pub mod coord;
pub mod rest_methods;

#[cfg(feature = "overpass")]
pub mod overpass;
