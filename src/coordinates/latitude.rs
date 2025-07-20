use std::ops::{Deref, RangeInclusive};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType};

pub const LATITUDE_RANGE: RangeInclusive<CoordinateType> = -90.0..=90.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Latitude {
    latitude: CoordinateType,
}

impl Latitude {
    /// Constructs a new [`Latitude`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the latitude provided is outside of the [`LATITUDE_RANGE`].
    #[inline]
    pub fn new(latitude: CoordinateType) -> Result<Self, coordinates::error::Error> {
        if Self::is_valid_latitude(&latitude) {
            Ok(Self { latitude })
        } else {
            Err(coordinates::error::Error::OutOfRange)
        }
    }

    /// Check if the supplied latitude is in the [`LATITUDE_RANGE`].
    #[inline]
    pub fn is_valid_latitude(latitude: &CoordinateType) -> bool {
        LATITUDE_RANGE.contains(&latitude)
    }

    /// Get the internal latitude.
    #[inline]
    pub fn value(&self) -> CoordinateType {
        self.latitude
    }
}

impl Deref for Latitude {
    type Target = CoordinateType;

    fn deref(&self) -> &Self::Target {
        &self.latitude
    }
}

#[cfg(test)]
mod latitude_test {
    use crate::coordinates::latitude::Latitude;

    #[test]
    fn in_range() {
        assert!(Latitude::new(0.0).is_ok())
    }

    #[test]
    fn in_range_lower_edge() {
        assert!(Latitude::new(-90.0).is_ok())
    }

    #[test]
    fn in_range_upper_edge() {
        assert!(Latitude::new(90.0).is_ok())
    }

    #[test]
    fn out_range_lower_edge() {
        assert!(Latitude::new(-90.1).is_err())
    }

    #[test]
    fn out_range_upper_edge() {
        assert!(Latitude::new(90.1).is_err())
    }

    #[test]
    fn out_range_lower() {
        assert!(Latitude::new(-160.0).is_err())
    }

    #[test]
    fn out_range_upper() {
        assert!(Latitude::new(160.0).is_err())
    }

    #[test]
    fn value() {
        let latitude = Latitude::new(2.0).unwrap();

        assert_eq!(2.0, latitude.value());
    }

    #[test]
    fn deref() {
        let latitude = Latitude::new(2.0).unwrap();

        assert_eq!(2.0, *latitude);
    }
}
