use std::ops::{Deref, RangeInclusive};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType};

pub const LONGITUDE_RANGE: RangeInclusive<CoordinateType> = -180.0..=180.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Longitude {
    longitude: CoordinateType,
}

impl Longitude {
    /// Constructs a new [`Longitude`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the longitude provided is outside of the [`LONGITUDE_RANGE`].
    #[inline]
    pub fn new(longitude: CoordinateType) -> Result<Self, coordinates::error::Error> {
        if Self::is_valid_longitude(&longitude) {
            Ok(Self { longitude })
        } else {
            Err(coordinates::error::Error::OutOfRange)
        }
    }

    /// Check if the supplied longitude is in the [`LONGITUDE_RANGE`].
    #[inline]
    pub fn is_valid_longitude(longitude: &CoordinateType) -> bool {
        LONGITUDE_RANGE.contains(&longitude)
    }

    /// Get the internal longitude.
    #[inline]
    pub fn value(&self) -> CoordinateType {
        self.longitude
    }
}

impl Deref for Longitude {
    type Target = CoordinateType;

    fn deref(&self) -> &Self::Target {
        &self.longitude
    }
}

#[cfg(test)]
mod longitude_test {
    use crate::coordinates::longitude::Longitude;

    #[test]
    fn in_range() {
        assert!(Longitude::new(0.0).is_ok())
    }

    #[test]
    fn in_range_lower_edge() {
        assert!(Longitude::new(-180.0).is_ok())
    }

    #[test]
    fn in_range_upper_edge() {
        assert!(Longitude::new(180.0).is_ok())
    }

    #[test]
    fn out_range_lower_edge() {
        assert!(Longitude::new(-180.1).is_err())
    }

    #[test]
    fn out_range_upper_edge() {
        assert!(Longitude::new(180.1).is_err())
    }

    #[test]
    fn out_range_lower() {
        assert!(Longitude::new(-360.0).is_err())
    }

    #[test]
    fn out_range_upper() {
        assert!(Longitude::new(360.0).is_err())
    }

    #[test]
    fn value() {
        let longitude = Longitude::new(2.0).unwrap();

        assert_eq!(2.0, longitude.value());
    }

    #[test]
    fn deref() {
        let longitude = Longitude::new(2.0).unwrap();

        assert_eq!(2.0, *longitude);
    }
}
