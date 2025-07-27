use crate::coord::CoordinateType;

pub trait Normalized {
    const MIN: CoordinateType;
    const MAX: CoordinateType;
    const SPAN: CoordinateType = Self::MAX - Self::MIN;

    fn normalized(value: CoordinateType) -> CoordinateType {
        debug_assert!(Self::SPAN > 0.0);

        (value - Self::MIN).rem_euclid(Self::SPAN) + Self::MIN
    }
}
