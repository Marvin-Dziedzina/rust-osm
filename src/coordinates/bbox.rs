use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::coordinates::{self, CoordinateType, coordinates::Coordinates};

/// A BBox or Bounding Box.
///
/// See https://wiki.openstreetmap.org/wiki/Bounding_box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BBox {
    south_west: Coordinates,
    north_east: Coordinates,
}

impl BBox {
    /// Construct a [`BBox`] from the south_west(lower left) and north_east(upper right) coordinate.
    ///
    /// # Error
    ///
    /// Returns a [`coordinates::error::Error::InvalidBBox`] if south_west > north_east.
    pub fn new(
        south_west: Coordinates,
        north_east: Coordinates,
    ) -> Result<Self, coordinates::error::Error> {
        if south_west < north_east {
            Ok(Self {
                south_west,
                north_east,
            })
        } else {
            Err(coordinates::error::Error::InvalidCornerOrder((
                south_west, north_east,
            )))
        }
    }

    /// Create a new [`BBox`] without checking if south_west ist south-west from north_east.
    pub fn from_unchecked(south_west: Coordinates, north_east: Coordinates) -> Self {
        Self {
            south_west,
            north_east,
        }
    }

    /// Return the lower left coordinate.
    pub fn south_west(&self) -> &Coordinates {
        &self.south_west
    }

    /// Return the upper right coordinate.
    pub fn north_east(&self) -> &Coordinates {
        &self.north_east
    }

    /// Get if a [`Coordinates`] is inside the [`BBox`].
    pub fn contains(&self, coord: &Coordinates) -> bool {
        self.south_west() <= coord && self.north_east() >= coord
    }

    /// Get the [`BBox`] width.
    pub fn width(&self) -> CoordinateType {
        *self.north_east().longitude() - *self.south_west().longitude()
    }

    /// Get the [`BBox`] height.
    pub fn height(&self) -> CoordinateType {
        *self.north_east().latitude() - *self.south_west().latitude()
    }

    /// Get the [`BBox`] area.
    pub fn area(&self) -> CoordinateType {
        self.width() * self.height()
    }

    /// Get the [`Coordinates`] of the center of this [`BBox`].
    pub fn center(&self) -> Coordinates {
        self.south_west().clone()
            + Coordinates::from_clamped(self.height() / 2.0, self.width() / 2.0)
    }

    /// Expand the [`BBox`] by delta_coord in all directions evenly.
    pub fn expand(&mut self, delta_coord: &Coordinates) {
        let half_delta_coord = delta_coord.clone() / 2.0;

        self.south_west -= half_delta_coord;
        self.north_east += half_delta_coord;
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

impl PartialEq for BBox {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
}

impl PartialOrd for BBox {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.area().partial_cmp(&other.area())
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

#[cfg(test)]
mod bbox_test {
    use crate::coordinates::{CoordinateType, bbox::BBox, coordinates::Coordinates};

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
            Coordinates::from_clamped(1.0, 0.0),
            Coordinates::from_clamped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(bbox.width(), 2.0);
        assert_eq!(bbox.height(), 1.0)
    }

    #[test]
    fn area() {
        let bbox = BBox::new(
            Coordinates::from_clamped(0.0, 0.0),
            Coordinates::from_clamped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(bbox.area(), bbox.width() * bbox.height());
    }

    #[test]
    fn center() {
        let bbox = BBox::new(
            Coordinates::from_clamped(0.0, 0.0),
            Coordinates::from_clamped(2.0, 2.0),
        )
        .unwrap();

        assert_eq!(bbox.center(), Coordinates::from_clamped(1.0, 1.0));
    }

    #[test]
    fn expand() {
        let mut bbox = BBox::new(
            Coordinates::from_clamped(0.0, 0.0),
            Coordinates::from_clamped(1.0, 1.0),
        )
        .unwrap();

        bbox.expand(&Coordinates::from_clamped(1.0, 1.0));

        assert_eq!(
            bbox,
            BBox::new(
                Coordinates::from_clamped(-0.5, -0.5),
                Coordinates::from_clamped(1.5, 1.5)
            )
            .unwrap()
        )
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
