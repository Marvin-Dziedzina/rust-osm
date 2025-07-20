use serde::{Deserialize, Serialize};

use crate::coordinates::{CoordinateType, coordinate::Coordinate};

/// A BBox or Bounding Box.
///
/// See https://wiki.openstreetmap.org/wiki/Bounding_box
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BBox {
    south_west: Coordinate,
    north_east: Coordinate,
}

impl BBox {
    /// Construct a [`BBox`] from the south_west(lower left) and north_east(upper right) coordinate.
    #[inline]
    pub fn new(south_west: Coordinate, north_east: Coordinate) -> Self {
        Self {
            south_west,
            north_east,
        }
    }

    /// Return the lower left coordinate.
    #[inline]
    pub fn south_west(&self) -> &Coordinate {
        &self.south_west
    }

    /// Return the upper right coordinate.
    #[inline]
    pub fn north_east(&self) -> &Coordinate {
        &self.north_east
    }

    /// Get the internal [`Coordinate`]s as a tuple with the ordering of (south, west, north, east).
    #[inline]
    pub fn tuple(
        &self,
    ) -> (
        CoordinateType,
        CoordinateType,
        CoordinateType,
        CoordinateType,
    ) {
        (
            self.south_west().latitude().value(),
            self.south_west().longitude().value(),
            self.north_east().latitude().value(),
            self.north_east().longitude().value(),
        )
    }
}

#[cfg(test)]
mod bbox_test {
    use crate::coordinates::{bbox::BBox, coordinate::Coordinate};

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
        let tuple = bbox.tuple();

        assert_eq!(1.0, tuple.0);
        assert_eq!(1.5, tuple.1);
        assert_eq!(2.0, tuple.2);
        assert_eq!(2.5, tuple.3);
    }

    fn get_bbox() -> BBox {
        BBox::new(
            Coordinate::from_value(1.0, 1.5).unwrap(),
            Coordinate::from_value(2.0, 2.5).unwrap(),
        )
    }
}
