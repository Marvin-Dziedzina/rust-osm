pub mod bbox;
pub mod coordinates;
pub mod error;
pub mod latitude;
pub mod longitude;

#[cfg(feature = "coordinate_f32")]
pub type CoordinateType = f32;
#[cfg(feature = "coordinate_f64")]
pub type CoordinateType = f64;
