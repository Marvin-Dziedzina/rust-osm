use std::{
    fmt::Display,
    ops::{Div, Mul},
};

use serde::{Deserialize, Serialize};

use crate::coord::{self, CoordinateType, coordinates::Coordinates};

/// A BBox or Bounding Box.
///
/// See <https://wiki.openstreetmap.org/wiki/Bounding_box>
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct BBox {
    south_west: Coordinates,
    north_east: Coordinates,
}

impl BBox {
    /// Construct a [`BBox`] from the south_west(lower left) and north_east(upper right) coordinate.
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::InvalidCornerOrder`] if south_west > north_east.
    pub fn new(
        south_west: Coordinates,
        north_east: Coordinates,
    ) -> Result<Self, coord::error::Error> {
        if south_west.latitude() < north_east.latitude()
            && south_west.longitude() < north_east.longitude()
        {
            Ok(Self {
                south_west,
                north_east,
            })
        } else {
            Err(coord::error::Error::InvalidCornerOrder((
                south_west, north_east,
            )))
        }
    }

    /// Create a new [`BBox`] without checking if south_west is south-west from north_east.
    pub fn from_unchecked(south_west: Coordinates, north_east: Coordinates) -> Self {
        Self {
            south_west,
            north_east,
        }
    }

    pub fn from_wrapped(
        south_west_latitude: CoordinateType,
        south_west_longitude: CoordinateType,
        north_east_latitude: CoordinateType,
        north_east_longitude: CoordinateType,
    ) -> Self {
        Self {
            south_west: Coordinates::from_wrapped(south_west_latitude, south_west_longitude),
            north_east: Coordinates::from_wrapped(north_east_latitude, north_east_longitude),
        }
    }

    /// Return the lower left coordinate.
    pub fn south_west(&self) -> Coordinates {
        self.south_west
    }

    /// Return the upper right coordinate.
    pub fn north_east(&self) -> Coordinates {
        self.north_east
    }

    /// Get latitude degrees.
    pub fn delta_lat_deg(&self) -> CoordinateType {
        CoordinateType::from(self.north_east().latitude())
            - CoordinateType::from(self.south_west().latitude())
    }

    /// Get longitude degrees.
    pub fn delta_lon_deg(&self) -> CoordinateType {
        CoordinateType::from(self.north_east().longitude())
            - CoordinateType::from(self.south_west().longitude())
    }

    /// Get latitude in rad.
    pub fn delta_lat_rad(&self) -> CoordinateType {
        Self::deg_to_rad(self.delta_lat_deg())
    }

    /// Get longitude in rad.
    pub fn delta_lon_rad(&self) -> CoordinateType {
        Self::deg_to_rad(self.delta_lon_deg())
    }

    /// Get latitude in m.
    pub fn height_m(&self) -> CoordinateType {
        todo!()
    }

    /// Get longitude in m.
    pub fn width_m(&self) -> CoordinateType {
        todo!()
    }

    /// Get the corners of the [`BBox`].
    ///
    /// (south_west.latitude, south_west.longitude, north_east.latitude, north_east.longitude)
    pub fn corners(
        &self,
    ) -> (
        CoordinateType,
        CoordinateType,
        CoordinateType,
        CoordinateType,
    ) {
        (
            self.south_west.latitude().value(),
            self.south_west.longitude().value(),
            self.north_east.latitude().value(),
            self.north_east.longitude().value(),
        )
    }

    /// Get the [`BBox`] area in deg2.
    pub fn area_deg2(&self) -> CoordinateType {
        self.delta_lon_deg() * self.delta_lat_deg()
    }

    /// Get the [`BBox`] area in m2.
    pub fn area_m2(&self) -> CoordinateType {
        self.width_m() * self.height_m()
    }

    /// Get the [`Coordinates`] of the center of this [`BBox`].
    pub fn center(&self) -> Coordinates {
        self.south_west()
            + Coordinates::from_wrapped(self.delta_lat_deg() / 2.0, self.delta_lon_deg() / 2.0)
    }

    /// Get if a [`Coordinates`] is inside the [`BBox`].
    ///
    /// This function is inclusive.
    pub fn contains(&self, p: &Coordinates) -> bool {
        let lat = p.latitude().value();
        let lon = p.longitude().value();

        Self::between_inclusive(
            lat,
            self.south_west.latitude().value(),
            self.north_east.latitude().value(),
        ) && Self::between_inclusive(
            lon,
            self.south_west.longitude().value(),
            self.north_east.longitude().value(),
        )
    }

    /// Get if a [`BBox`] is inside the [`BBox`].
    ///
    /// This function is inclusive.
    pub fn contains_bbox(&self, other: &Self) -> bool {
        self.contains(&other.south_west()) && self.contains(&other.north_east())
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let (a_s, a_w) = (
            self.south_west.latitude().value(),
            self.south_west.longitude().value(),
        );
        let (a_n, a_e) = (
            self.north_east.latitude().value(),
            self.north_east.longitude().value(),
        );
        let (b_s, b_w) = (
            other.south_west.latitude().value(),
            other.south_west.longitude().value(),
        );
        let (b_n, b_e) = (
            other.north_east.latitude().value(),
            other.north_east.longitude().value(),
        );

        Self::overlaps_1d(a_s, a_n, b_s, b_n) && Self::overlaps_1d(a_w, a_e, b_w, b_e)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        };

        let sw_lat = self
            .south_west
            .latitude()
            .value()
            .max(other.south_west.latitude().value());
        let sw_lon = self
            .south_west
            .longitude()
            .value()
            .max(other.south_west.longitude().value());
        let ne_lat = self
            .north_east
            .latitude()
            .value()
            .min(other.north_east.latitude().value());
        let ne_lon = self
            .north_east
            .longitude()
            .value()
            .min(other.north_east.longitude().value());

        Some(BBox::from_wrapped(sw_lat, sw_lon, ne_lat, ne_lon))
    }

    fn between_inclusive(v: CoordinateType, lo: CoordinateType, hi: CoordinateType) -> bool {
        v >= lo && v <= hi
    }

    fn overlaps_1d(
        a_min: CoordinateType,
        a_max: CoordinateType,
        b_min: CoordinateType,
        b_max: CoordinateType,
    ) -> bool {
        a_min <= b_max && b_min <= a_max
    }

    #[cfg(feature = "coordinate_f32")]
    pub fn deg_to_rad(deg: CoordinateType) -> CoordinateType {
        deg * std::f32::consts::PI / 180.0
    }

    #[cfg(feature = "coordinate_f64")]
    pub fn deg_to_rad(deg: CoordinateType) -> CoordinateType {
        deg * std::f64::consts::PI / 180.0
    }

    #[cfg(feature = "coordinate_f32")]
    pub fn rad_to_deg(rad: CoordinateType) -> CoordinateType {
        rad * 180.0 / std::f32::consts::PI
    }

    #[cfg(feature = "coordinate_f64")]
    pub fn rad_to_deg(rad: CoordinateType) -> CoordinateType {
        rad * 180.0 / std::f64::consts::PI
    }
}

impl From<BBox>
    for (
        CoordinateType,
        CoordinateType,
        CoordinateType,
        CoordinateType,
    )
{
    fn from(value: BBox) -> Self {
        (
            value.south_west().latitude().value(),
            value.south_west().longitude().value(),
            value.north_east().latitude().value(),
            value.north_east().longitude().value(),
        )
    }
}

impl<T: Into<CoordinateType>> TryFrom<(T, T, T, T)> for BBox {
    type Error = coord::error::Error;

    fn try_from(tuple: (T, T, T, T)) -> Result<Self, Self::Error> {
        Self::new(
            Coordinates::from_value(tuple.0.into(), tuple.1.into())?,
            Coordinates::from_value(tuple.2.into(), tuple.3.into())?,
        )
    }
}

impl PartialEq for BBox {
    fn eq(&self, other: &Self) -> bool {
        self.south_west == other.south_west && self.north_east == other.north_east
    }
}

impl Eq for BBox {}

impl PartialOrd for BBox {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        let a_in_b = other.contains_bbox(self);
        let b_in_a = self.contains_bbox(other);

        match (a_in_b, b_in_a) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}

impl Display for BBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let south_west = self.south_west();
        let north_east = self.north_east();
        write!(
            f,
            "(( South: {}, West: {} ), ( North: {}, East: {} ))",
            south_west.latitude(),
            south_west.longitude(),
            north_east.latitude(),
            north_east.longitude()
        )
    }
}

impl<T: Into<CoordinateType>> Mul<T> for BBox {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::from_unchecked(self.south_west * rhs, self.north_east * rhs)
    }
}

impl<T: Into<CoordinateType>> Div<T> for BBox {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::from_unchecked(self.south_west / rhs, self.north_east / rhs)
    }
}

#[cfg(test)]
mod bbox_test {
    use crate::coord::{CoordinateType, bbox::BBox, coordinates::Coordinates};

    #[test]
    fn south_west() {
        let bbox = get_bbox();
        let south_west = bbox.south_west();

        assert_eq!(1.0, south_west.latitude().value());
        assert_eq!(1.5, south_west.longitude().value());
    }

    #[test]
    fn north_east() {
        let bbox = get_bbox();
        let north_east = bbox.north_east();

        assert_eq!(2.0, north_east.latitude().value());
        assert_eq!(2.5, north_east.longitude().value());
    }

    #[test]
    fn tuple() {
        let bbox = get_bbox();
        let tuple: (
            CoordinateType,
            CoordinateType,
            CoordinateType,
            CoordinateType,
        ) = bbox.into();

        assert_eq!(1.0, tuple.0);
        assert_eq!(1.5, tuple.1);
        assert_eq!(2.0, tuple.2);
        assert_eq!(2.5, tuple.3);
    }

    #[test]
    fn width_height() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(1.0, 0.0),
            Coordinates::from_wrapped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(bbox.delta_lon_deg(), 2.0);
        assert_eq!(bbox.delta_lat_deg(), 1.0)
    }

    #[test]
    fn area() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(0.0, 0.0),
            Coordinates::from_wrapped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(
            bbox.area_deg2(),
            bbox.delta_lon_deg() * bbox.delta_lat_deg()
        );
    }

    #[test]
    fn center() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(0.0, 0.0),
            Coordinates::from_wrapped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(bbox.center(), Coordinates::from_wrapped(1.0, 1.0));
    }

    #[test]
    fn contains() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(0.0, 0.0),
            Coordinates::from_wrapped(50.0, 50.0),
        )
        .unwrap();

        assert!(bbox.contains(&Coordinates::from_wrapped(25.0, 25.0)));
    }

    #[test]
    fn contains_edge() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(0.0, 0.0),
            Coordinates::from_wrapped(50.0, 50.0),
        )
        .unwrap();

        assert!(bbox.contains(&Coordinates::from_wrapped(50.0, 0.0)));
    }

    #[test]
    fn contains_fail() {
        let bbox = BBox::new(
            Coordinates::from_wrapped(0.0, 0.0),
            Coordinates::from_wrapped(50.0, 50.0),
        )
        .unwrap();

        assert!(!bbox.contains(&Coordinates::from_wrapped(-1.0, 0.0)));
    }

    #[test]
    fn contains_bbox() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);

        assert!(bbox.contains_bbox(&BBox::from_wrapped(10.0, 20.0, 49.0, 40.0)));
    }

    #[test]
    fn contains_bbox_edge() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);

        assert!(bbox.contains_bbox(&BBox::from_wrapped(0.0, 0.0, 50.0, 50.0)));
    }

    #[test]
    fn contains_bbox_fail() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);

        assert!(!bbox.contains_bbox(&BBox::from_wrapped(-1.0, -1.0, 50.0, 50.0)));
    }

    #[test]
    fn intersects() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(49.0, 49.0, 80.0, 80.0);

        assert!(bbox.intersects(&other));
        assert!(other.intersects(&bbox));
    }

    #[test]
    fn intersects_no_intersect() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(50.1, 50.1, 80.0, 80.0);

        assert!(!bbox.intersects(&other));
        assert!(!other.intersects(&bbox));
    }

    #[test]
    fn intersects_eq_intersect() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);

        assert_eq!(bbox, other);
        assert!(bbox.intersects(&other));
        assert!(other.intersects(&bbox));
    }

    #[test]
    fn intersection() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(45.0, 45.0, 80.0, 80.0);

        assert_eq!(
            bbox.intersection(&other).unwrap(),
            BBox::from_wrapped(45.0, 45.0, 50.0, 50.0)
        );
        assert_eq!(
            other.intersection(&bbox).unwrap(),
            BBox::from_wrapped(45.0, 45.0, 50.0, 50.0)
        );
    }

    #[test]
    fn intersection_no_intersection() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(50.1, 45.0, 80.0, 80.0);

        assert!(bbox.intersection(&other).is_none());
    }

    #[test]
    fn intersection_eq_intersection() {
        let bbox = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);
        let other = BBox::from_wrapped(0.0, 0.0, 50.0, 50.0);

        let intersection = bbox.intersection(&other).unwrap();
        assert_eq!(intersection, bbox);
        assert_eq!(intersection, other);
    }

    #[test]
    fn partial_eq_eq() {
        let bbox1 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(1.0, 1.0).unwrap(),
        )
        .unwrap();
        let bbox2 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(1.0, 1.0).unwrap(),
        )
        .unwrap();

        assert_eq!(bbox1, bbox2);
    }

    #[test]
    fn partial_eq_ne() {
        let bbox1 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(1.0, 1.0).unwrap(),
        )
        .unwrap();
        let bbox2 = BBox::new(
            Coordinates::from_value(0.0, 1.0).unwrap(),
            Coordinates::from_value(2.0, 2.0).unwrap(),
        )
        .unwrap();

        assert_ne!(bbox1, bbox2);
    }

    #[test]
    fn partial_ord_greater() {
        let bbox1 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(2.0, 2.0).unwrap(),
        )
        .unwrap();
        let bbox2 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(1.0, 1.0).unwrap(),
        )
        .unwrap();

        assert!(bbox1 > bbox2);
    }

    #[test]
    fn partial_ord_less() {
        let bbox1 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(1.0, 1.0).unwrap(),
        )
        .unwrap();
        let bbox2 = BBox::new(
            Coordinates::from_value(0.0, 0.0).unwrap(),
            Coordinates::from_value(2.0, 2.0).unwrap(),
        )
        .unwrap();

        assert!(bbox1 < bbox2);
    }

    fn get_bbox() -> BBox {
        BBox::new(
            Coordinates::from_value(1.0, 1.5).unwrap(),
            Coordinates::from_value(2.0, 2.5).unwrap(),
        )
        .unwrap()
    }
}
