use std::ops::RangeInclusive;

use crate::coord::{CoordinateType, coordinates::Coordinates};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value is out of range")]
    OutOfRange((CoordinateType, RangeInclusive<CoordinateType>)),
    #[error("south_west must be more south-west than north_east")]
    InvalidCornerOrder((Coordinates, Coordinates)),
}
