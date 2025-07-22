use std::{
    fmt::Display,
    ops::{Add, Deref, Div, Mul, RangeInclusive, Sub},
};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType};

pub const LONGITUDE_RANGE: RangeInclusive<CoordinateType> = -180.0..=180.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Longitude {
    longitude: CoordinateType,
}

impl Longitude {
    /// Constructs a new [`Longitude`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the longitude provided is outside of the [`LONGITUDE_RANGE`].
    pub fn new(longitude: CoordinateType) -> Result<Self, coordinates::error::Error> {
        if Self::is_valid_longitude(&longitude) {
            Ok(Self { longitude })
        } else {
            Err(coordinates::error::Error::OutOfRange)
        }
    }

    pub fn from_unchecked(longitude: CoordinateType) -> Self {
        Self { longitude }
    }

    /// Construct a new [`Longitude`] and clamp longitude to the [`LONGITUDE_RANGE`].
    pub fn from_clamped(longitude: CoordinateType) -> Self {
        if LONGITUDE_RANGE.contains(&longitude) {
            Self {
                longitude: longitude,
            }
        } else {
            Self {
                longitude: longitude.clamp(*LONGITUDE_RANGE.start(), *LONGITUDE_RANGE.end()),
            }
        }
    }

    /// Check if the supplied longitude is in the [`LONGITUDE_RANGE`].
    pub fn is_valid_longitude(longitude: &CoordinateType) -> bool {
        LONGITUDE_RANGE.contains(&longitude)
    }

    /// Get the internal longitude.
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

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<CoordinateType> for Longitude {
    type Error = coordinates::error::Error;

    fn try_from(longitude: CoordinateType) -> Result<Self, Self::Error> {
        Self::new(longitude)
    }
}

impl From<Longitude> for CoordinateType {
    fn from(longitude: Longitude) -> Self {
        longitude.longitude
    }
}

impl Add for Longitude {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.longitude + rhs.longitude)
    }
}

impl Sub for Longitude {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.longitude - rhs.longitude)
    }
}

impl Mul for Longitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.longitude * rhs.longitude)
    }
}

impl Div for Longitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.longitude / rhs.longitude)
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

    #[test]
    fn partial_ord() {
        let longitude1 = Longitude::new(1.0).unwrap();
        let longitude2 = Longitude::new(2.0).unwrap();

        assert!(longitude1 < longitude2);
        assert!(!(longitude1 > longitude2));
    }
}
