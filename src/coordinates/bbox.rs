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
