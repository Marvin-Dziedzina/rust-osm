use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType, latitude::Latitude, longitude::Longitude};

/// A single point on a plane.
///
/// See https://wiki.openstreetmap.org/wiki/Coordinates
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinates {
    latitude: Latitude,
    longitude: Longitude,
}

impl Coordinates {
    /// Construct a new [`Coordinate`] from [`CoordinateType`].
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    /// Construct a new [`Coordinate`] from [`CoordinateType`].
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::OutOfRange`] if the latitude or the longitude is out of range.
    pub fn from_value(
        latitude: CoordinateType,
        longitude: CoordinateType,
    ) -> Result<Self, coordinates::error::Error> {
        Ok(Self::new(
            Latitude::new(latitude)?,
            Longitude::new(longitude)?,
        ))
    }

    /// Construct a new unchecked [`Coordinate`] from [`CoordinateType`].
    pub fn from_unchecked(latitude: CoordinateType, longitude: CoordinateType) -> Self {
        Self::new(
            Latitude::from_unchecked(latitude),
            Longitude::from_unchecked(longitude),
        )
    }

    /// Construct a new [`Coordinate`] from latitude and longitude that will get clamped to a valid value.
    pub fn from_clamped(latitude: CoordinateType, longitude: CoordinateType) -> Self {
        Self::new(
            Latitude::from_clamped(latitude),
            Longitude::from_clamped(longitude),
        )
    }

    /// [`Latitude`] of this [`Coordinate`].
    ///
    /// [`Latitude`] is the y coordinate.
    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    /// [`Longitude`] of this [`Coordinate`].
    ///
    /// [`Longitude`] is the x coordinate.
    pub fn longitude(&self) -> Longitude {
        self.longitude
    }
}

impl From<Coordinates> for (CoordinateType, CoordinateType) {
    fn from(value: Coordinates) -> Self {
        (value.latitude().value(), value.longitude().value())
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude && self.longitude == other.longitude
    }
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        let lat_cmp = self.latitude.partial_cmp(&other.latitude)?;
        let lon_cmp = self.longitude.partial_cmp(&other.longitude)?;

        match (lat_cmp, lon_cmp) {
            (Ordering::Less, Ordering::Less | Ordering::Equal)
            | (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Greater, Ordering::Greater | Ordering::Equal)
            | (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( Lat: {}, Lon: {} )", self.latitude(), self.longitude())
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.latitude + rhs.latitude, self.longitude + rhs.longitude)
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.latitude = self.latitude + rhs.latitude;
        self.longitude = self.longitude + rhs.longitude;
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.latitude - rhs.latitude, self.longitude - rhs.longitude)
    }
}

impl SubAssign for Coordinates {
    fn sub_assign(&mut self, rhs: Self) {
        self.latitude = self.latitude - rhs.latitude;
        self.longitude = self.longitude - rhs.longitude;
    }
}

impl Mul for Coordinates {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.latitude * rhs.latitude, self.longitude * rhs.longitude)
    }
}

impl Mul<CoordinateType> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: CoordinateType) -> Self::Output {
        Self::from_clamped(self.latitude.value() * rhs, self.longitude.value() * rhs)
    }
}

impl Div for Coordinates {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.latitude / rhs.latitude, self.longitude / rhs.longitude)
    }
}

impl Div<CoordinateType> for Coordinates {
    type Output = Self;

    fn div(self, rhs: CoordinateType) -> Self::Output {
        Self::from_clamped(self.latitude.value() / rhs, self.longitude.value() / rhs)
    }
}

#[cfg(test)]
mod coordinate_test {
    use crate::coordinates::{CoordinateType, coordinates::Coordinates};

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
        let tuple: (CoordinateType, CoordinateType) = coordinate.into();

        assert_eq!(1.0, tuple.0);
        assert_eq!(2.0, tuple.1);
    }

    #[test]
    fn partial_eq_eq() {
        let coord1 = Coordinates::from_value(1.0, 1.0).unwrap();
        let coord2 = Coordinates::from_value(1.0, 1.0).unwrap();

        assert_eq!(coord1, coord2);
    }

    #[test]
    fn partial_eq_ne() {
        let coord1 = Coordinates::from_value(1.0, 1.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert_ne!(coord1, coord2);
    }

    #[test]
    fn partial_eq_lat_ne() {
        let coord1 = Coordinates::from_value(2.0, 1.0).unwrap();
        let coord2 = Coordinates::from_value(1.0, 1.0).unwrap();

        assert_ne!(coord1, coord2);
    }

    #[test]
    fn partial_eq_lon_ne() {
        let coord1 = Coordinates::from_value(1.0, 2.0).unwrap();
        let coord2 = Coordinates::from_value(1.0, 1.0).unwrap();

        assert_ne!(coord1, coord2);
    }

    #[test]
    fn partial_ord_greater_less() {
        let coord1 = Coordinates::from_value(1.0, 1.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert!(coord1 < coord2);
        assert!(!(coord1 > coord2));
    }

    #[test]
    fn partial_ord_partial_greater_lat() {
        let coord1 = Coordinates::from_value(3.0, 2.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert!(coord1 > coord2);
        assert!(!(coord1 < coord2));
    }

    #[test]
    fn partial_ord_partial_non_cmp_lat() {
        let coord1 = Coordinates::from_value(3.0, 1.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert!(!(coord1 > coord2));
    }

    #[test]
    fn partial_ord_partial_greater_lon() {
        let coord1 = Coordinates::from_value(2.0, 3.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert!(coord1 > coord2);
        assert!(!(coord1 < coord2));
    }

    #[test]
    fn partial_ord_partial_non_cmp_lon() {
        let coord1 = Coordinates::from_value(1.0, 3.0).unwrap();
        let coord2 = Coordinates::from_value(2.0, 2.0).unwrap();

        assert!(!(coord1 > coord2));
    }

    fn get_coordinate() -> Coordinates {
        Coordinates::from_value(1.0, 2.0).unwrap()
    }
}
