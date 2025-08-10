use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeInclusive, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::coord::{self, CoordinateType, normalize::Normalized};

pub const LATITUDE_RANGE: RangeInclusive<CoordinateType> = -90.0..=90.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Latitude(CoordinateType);

impl Latitude {
    /// Constructs a new [`Latitude`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the latitude provided is outside of the [`LATITUDE_RANGE`].
    pub fn new(latitude: CoordinateType) -> Result<Self, coord::error::Error> {
        if Self::is_valid(latitude) {
            Ok(Self(latitude))
        } else {
            Err(coord::error::Error::OutOfRange((latitude, LATITUDE_RANGE)))
        }
    }

    /// Construct a new unchecked [`Latitude`]. latitude should be in [`LATITUDE_RANGE`].
    pub const fn from_unchecked(latitude: CoordinateType) -> Self {
        Self(latitude)
    }

    /// Construct a new [`Latitude`] and clamp latitude to the [`LATITUDE_RANGE`].
    pub fn from_clamped(latitude: CoordinateType) -> Self {
        Self(latitude.clamp(*LATITUDE_RANGE.start(), *LATITUDE_RANGE.end()))
    }

    /// Check if the supplied latitude is in the [`LATITUDE_RANGE`].
    pub fn is_valid(latitude: CoordinateType) -> bool {
        LATITUDE_RANGE.contains(&latitude)
    }

    /// Get the internal latitude.
    pub fn value(&self) -> CoordinateType {
        self.0
    }
}

impl Normalized for Latitude {
    const MIN: CoordinateType = *LATITUDE_RANGE.start();

    const MAX: CoordinateType = *LATITUDE_RANGE.end();
}

impl Display for Latitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 >= 0.0 {
            write!(f, "{} °N", self.0)
        } else {
            write!(f, "{} °S", self.0.abs())
        }
    }
}

impl Eq for Latitude {}

impl Ord for Latitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Hash for Latitude {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.0 == 0.0 {
            0.0f64.to_bits()
        } else {
            self.0.to_bits()
        };

        bits.hash(state);
    }
}

impl TryFrom<CoordinateType> for Latitude {
    type Error = coord::error::Error;

    fn try_from(latitude: CoordinateType) -> Result<Self, Self::Error> {
        Self::new(latitude)
    }
}

impl From<Latitude> for CoordinateType {
    fn from(latitude: Latitude) -> Self {
        latitude.0
    }
}

impl<T: Into<CoordinateType>> Add<T> for Latitude {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::from_clamped(self.0 + rhs.into())
    }
}

impl<T: Into<CoordinateType>> AddAssign<T> for Latitude {
    fn add_assign(&mut self, rhs: T) {
        *self = Self::from_clamped(self.0 + rhs.into());
    }
}

impl<T: Into<CoordinateType>> Sub<T> for Latitude {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::from_clamped(self.0 - rhs.into())
    }
}

impl<T: Into<CoordinateType>> SubAssign<T> for Latitude {
    fn sub_assign(&mut self, rhs: T) {
        *self = Self::from_clamped(self.0 - rhs.into());
    }
}

impl<T: Into<CoordinateType>> Mul<T> for Latitude {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::from_clamped(self.0 * rhs.into())
    }
}

impl<T: Into<CoordinateType>> MulAssign<T> for Latitude {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::from_clamped(self.0 * rhs.into());
    }
}

impl<T: Into<CoordinateType>> Div<T> for Latitude {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::from_clamped(self.0 / rhs.into())
    }
}

impl<T: Into<CoordinateType>> DivAssign<T> for Latitude {
    fn div_assign(&mut self, rhs: T) {
        *self = Self::from_clamped(self.0 / rhs.into());
    }
}

impl Neg for Latitude {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_clamped(-self.0)
    }
}

#[cfg(test)]
mod latitude_test {
    use crate::coord::latitude::Latitude;

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
    fn partial_ord() {
        let latitude1 = Latitude::new(1.0).unwrap();
        let latitude2 = Latitude::new(2.0).unwrap();

        assert!(latitude1 < latitude2);
        assert!(!(latitude1 > latitude2));
    }

    #[test]
    fn neg() {
        assert_eq!(-Latitude::new(45.0).unwrap(), Latitude::new(-45.0).unwrap());
    }
}
