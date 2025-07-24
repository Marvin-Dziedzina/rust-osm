use std::{
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeInclusive, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType, normalize::Normalized};

pub const LONGITUDE_RANGE: RangeInclusive<CoordinateType> = -180.0..=180.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
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
        if Self::is_valid(longitude) {
            Ok(Self { longitude })
        } else {
            Err(coordinates::error::Error::OutOfRange((
                longitude,
                LONGITUDE_RANGE,
            )))
        }
    }

    /// Construct a new [`Longitude`]. longitude should be in [`LONGITUDE_RANGE`].
    pub const fn from_unchecked(longitude: CoordinateType) -> Self {
        Self { longitude }
    }

    /// Construct a new [`Longitude`] and wrap longitude to the [`LONGITUDE_RANGE`].
    pub fn from_wrapped(longitude: CoordinateType) -> Self {
        Self {
            longitude: Self::normalized(longitude),
        }
    }

    /// Check if the supplied longitude is in the [`LONGITUDE_RANGE`].
    pub fn is_valid(longitude: CoordinateType) -> bool {
        LONGITUDE_RANGE.contains(&longitude)
    }

    /// Get the internal longitude.
    pub const fn value(&self) -> CoordinateType {
        self.longitude
    }
}

impl Normalized for Longitude {
    const MIN: CoordinateType = *LONGITUDE_RANGE.start();
    const MAX: CoordinateType = *LONGITUDE_RANGE.end();
}

impl Display for Longitude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.longitude >= 0.0 {
            write!(f, "{} °E", self.longitude)
        } else {
            write!(f, "{} °W", self.longitude.abs())
        }
    }
}

impl Eq for Longitude {}

impl Ord for Longitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.longitude.total_cmp(&other.longitude)
    }
}

impl Hash for Longitude {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let bits = if self.longitude == 0.0 {
            0.0f64.to_bits()
        } else {
            self.longitude.to_bits()
        };

        bits.hash(state);
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

impl<T: Into<CoordinateType>> Add<T> for Longitude {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::from_wrapped(self.longitude + rhs.into())
    }
}

impl<T: Into<CoordinateType>> AddAssign<T> for Longitude {
    fn add_assign(&mut self, rhs: T) {
        *self = Self::from_wrapped(self.longitude + rhs.into());
    }
}

impl<T: Into<CoordinateType>> Sub<T> for Longitude {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Self::from_wrapped(self.longitude - rhs.into())
    }
}

impl<T: Into<CoordinateType>> SubAssign<T> for Longitude {
    fn sub_assign(&mut self, rhs: T) {
        *self = Self::from_wrapped(self.longitude - rhs.into());
    }
}

impl<T: Into<CoordinateType>> Mul<T> for Longitude {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::from_wrapped(self.longitude * rhs.into())
    }
}

impl<T: Into<CoordinateType>> MulAssign<T> for Longitude {
    fn mul_assign(&mut self, rhs: T) {
        *self = Self::from_wrapped(self.longitude * rhs.into());
    }
}

impl<T: Into<CoordinateType>> Div<T> for Longitude {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::from_wrapped(self.longitude / rhs.into())
    }
}

impl<T: Into<CoordinateType>> DivAssign<T> for Longitude {
    fn div_assign(&mut self, rhs: T) {
        *self = Self::from_wrapped(self.longitude / rhs.into());
    }
}

impl Neg for Longitude {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_wrapped(-self.longitude)
    }
}

#[cfg(test)]
mod longitude_test {
    use crate::coordinates::{
        CoordinateType,
        longitude::{LONGITUDE_RANGE, Longitude},
    };

    #[test]
    fn in_range() {
        assert!(Longitude::new(0.0).is_ok())
    }

    #[test]
    fn in_range_lower_edge() {
        assert!(Longitude::new(*LONGITUDE_RANGE.start()).is_ok())
    }

    #[test]
    fn in_range_upper_edge() {
        assert!(Longitude::new(*LONGITUDE_RANGE.end()).is_ok())
    }

    #[test]
    fn out_range_lower_edge() {
        assert!(Longitude::new(LONGITUDE_RANGE.start() - 0.1).is_err())
    }

    #[test]
    fn out_range_upper_edge() {
        assert!(Longitude::new(LONGITUDE_RANGE.end() + 0.1).is_err())
    }

    #[test]
    fn out_range_lower() {
        assert!(Longitude::new(LONGITUDE_RANGE.start() * 2.0).is_err())
    }

    #[test]
    fn out_range_upper() {
        assert!(Longitude::new(LONGITUDE_RANGE.end() * 2.0).is_err())
    }

    #[test]
    fn wrapped_zero() {
        assert_eq!(Longitude::from_wrapped(0.0).value(), 0.0);
    }

    #[test]
    fn wrapped_lower_edge() {
        assert_eq!(
            Longitude::from_wrapped(*LONGITUDE_RANGE.start()).value(),
            *LONGITUDE_RANGE.start()
        );
    }

    #[test]
    fn wrapped_upper_edge() {
        assert_eq!(
            Longitude::from_wrapped(*LONGITUDE_RANGE.end()).value(),
            -*LONGITUDE_RANGE.end()
        );
    }

    #[test]
    fn wrapped_over_lower_edge() {
        assert_eq!(
            round(Longitude::from_wrapped(*LONGITUDE_RANGE.start() - 0.1).value()),
            *LONGITUDE_RANGE.end() - 0.1
        );
    }

    #[test]
    fn wrapped_over_upper_edge() {
        assert_eq!(
            round(Longitude::from_wrapped(*LONGITUDE_RANGE.end() + 0.1).value()),
            *LONGITUDE_RANGE.start() + 0.1
        );
    }

    #[test]
    fn wrapped_lower() {
        assert_eq!(
            Longitude::from_wrapped(*LONGITUDE_RANGE.start() * 2.0).value(),
            0.0
        );
    }

    #[test]
    fn wrapped_upper() {
        assert_eq!(
            Longitude::from_wrapped(*LONGITUDE_RANGE.end() * 2.0).value(),
            0.0
        );
    }

    #[test]
    fn value() {
        let longitude = Longitude::new(2.0).unwrap();

        assert_eq!(2.0, longitude.value());
    }

    #[test]
    fn partial_ord() {
        let longitude1 = Longitude::new(1.0).unwrap();
        let longitude2 = Longitude::new(2.0).unwrap();

        assert!(longitude1 < longitude2);
        assert!(!(longitude1 > longitude2));
    }

    #[test]
    fn neg() {
        assert_eq!(
            -Longitude::new(45.0).unwrap(),
            Longitude::new(-45.0).unwrap()
        );
    }

    fn round(x: CoordinateType) -> CoordinateType {
        (x * 1e6).round() / 1e6
    }
}
