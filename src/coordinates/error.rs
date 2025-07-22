use crate::coordinates::coordinates::Coordinates;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value is out of range")]
    OutOfRange,
    #[error("south_west must be more south-west than north_east")]
    InvalidCornerOrder((Coordinates, Coordinates)),
}
