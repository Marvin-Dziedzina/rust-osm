use std::{
    fmt::Display,
    ops::{Add, Deref, Div, Mul, RangeInclusive, Sub},
};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType};

pub const LATITUDE_RANGE: RangeInclusive<CoordinateType> = -90.0..=90.0;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Latitude {
    latitude: CoordinateType,
}

impl Latitude {
    /// Constructs a new [`Latitude`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the latitude provided is outside of the [`LATITUDE_RANGE`].
    pub fn new(latitude: CoordinateType) -> Result<Self, coordinates::error::Error> {
        if Self::is_valid_latitude(&latitude) {
            Ok(Self { latitude })
        } else {
            Err(coordinates::error::Error::OutOfRange)
        }
    }

    /// Construct a new unchecked [`Latitude`].
    pub fn from_unchecked(latitude: CoordinateType) -> Self {
        Self { latitude }
    }

    /// Construct a new [`Latitude`] and clamp latitude to the [`LATITUDE_RANGE`].
    pub fn from_clamped(latitude: CoordinateType) -> Self {
        if LATITUDE_RANGE.contains(&latitude) {
            Self { latitude }
        } else {
            Self {
                latitude: latitude.clamp(*LATITUDE_RANGE.start(), *LATITUDE_RANGE.end()),
            }
        }
    }

    /// Check if the supplied latitude is in the [`LATITUDE_RANGE`].
    pub fn is_valid_latitude(latitude: &CoordinateType) -> bool {
        LATITUDE_RANGE.contains(&latitude)
    }

    /// Get the internal latitude.
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

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<CoordinateType> for Latitude {
    type Error = coordinates::error::Error;

    fn try_from(latitude: CoordinateType) -> Result<Self, Self::Error> {
        Self::new(latitude)
    }
}

impl From<Latitude> for CoordinateType {
    fn from(latitude: Latitude) -> Self {
        latitude.latitude
    }
}

impl Add for Latitude {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.latitude + rhs.latitude)
    }
}

impl Sub for Latitude {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.latitude - rhs.latitude)
    }
}

impl Mul for Latitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.latitude * rhs.latitude)
    }
}

impl Div for Latitude {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_clamped(self.latitude / rhs.latitude)
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

    #[test]
    fn partial_ord() {
        let latitude1 = Latitude::new(1.0).unwrap();
        let latitude2 = Latitude::new(2.0).unwrap();

        assert!(latitude1 < latitude2);
        assert!(!(latitude1 > latitude2));
    }
}
