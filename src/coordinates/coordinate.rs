use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType, latitude::Latitude, longitude::Longitude};

/// A single point on a plane.
///
/// See https://wiki.openstreetmap.org/wiki/Coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
    latitude: Latitude,
    longitude: Longitude,
}

impl Coordinate {
    /// Constructs a new [`Coordinate`] from [`CoordinateType`].
    #[inline]
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Constructs a new [`Coordinate`] from [`CoordinateType`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the latitude or the longitude is out of range.
    #[inline]
    pub fn from_value(
        latitude: CoordinateType,
        longitude: CoordinateType,
    ) -> Result<Self, coordinates::error::Error> {
        Ok(Self::new(
            Latitude::new(latitude)?,
            Longitude::new(longitude)?,
        ))
    }

    /// [`Latitude`] of this [`Coordinate`].
    ///
    /// [`Latitude`] is the y coordinate.
    #[inline]
    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    /// [`Longitude`] of this [`Coordinate`].
    ///
    /// [`Longitude`] is the x coordinate.
    #[inline]
    pub fn longitude(&self) -> Longitude {
        self.longitude
    }

    /// Get the internal latitude and longitude as a tuple.
    #[inline]
    pub fn tuple(&self) -> (CoordinateType, CoordinateType) {
        (self.latitude().value(), self.longitude().value())
    }
}

#[cfg(test)]
mod coordinate_test {
    use crate::coordinates::coordinate::Coordinate;

    #[test]
    fn latitude() {
        let coordinate = get_coordinate();

        assert_eq!(1.0, coordinate.latitude().value());
    }

    #[test]
    fn longitude() {
        let coordinate = get_coordinate();

        assert_eq!(2.0, coordinate.longitude().value());
    }

    #[test]
    fn tuple() {
        let coordinate = get_coordinate();
        let tuple = coordinate.tuple();

        assert_eq!(1.0, tuple.0);
        assert_eq!(2.0, tuple.1);
    }

    fn get_coordinate() -> Coordinate {
        Coordinate::from_value(1.0, 2.0).unwrap()
    }
}
