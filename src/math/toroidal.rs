use super::Vec2;
use super::bounding::AxisAlignmentBoundingBox;

/// Computes the minimal analogous distance in a toroidal space of the specified length.
/// The maximun value for this minimal distance is never greater of length / 2.
pub fn min_distance(distance: f32, length: u32) -> f32 {
    let length = length as f32;
    let half_length = length / 2.0;
    if distance > half_length {
        if distance <= length { distance - length } else { half_length }
    }
    else if distance < -half_length {
        if distance >= -length { distance + length } else { -half_length }
    }
    else {
        distance
    }
}

/// Computes the mininal representation of a coordinate in a toroidal space of the specified length.
/// The computed coordinate is always between 0 and length.
pub fn min_coordinate(coordinate: f32, length: u32) -> f32 {
    if coordinate >= 0.0 {
        let uint_coordinate = coordinate as u32;
        let mod_coordinate = (uint_coordinate % length) as f32 + coordinate - uint_coordinate as f32;
        return if coordinate < 0.0 { mod_coordinate + length as f32} else { mod_coordinate }
    }
    else {
        let coordinate = -coordinate;
        let uint_coordinate = coordinate as u32;
        let mod_coordinate = (uint_coordinate % length) as f32 + coordinate - uint_coordinate as f32;
        let mod_coordinate = length as f32 - mod_coordinate;
        return if coordinate < 0.0 { mod_coordinate + length as f32} else { mod_coordinate }
    }
}

/// A 2D toroidal bounds definition
pub struct Bounds {
    pub width: u32,
    pub height: u32,
}

impl Bounds {
    pub fn new(width: u32, height: u32) -> Bounds {
        Bounds { width, height }
    }

    /// Similar to [`toroidal::min_distance()`](min_distance()) but for a 2D toroidal space.
    pub fn get_toroidal_distance(&self, distance: Vec2) -> Vec2 {
        Vec2::xy(
            min_distance(distance.x, self.width),
            min_distance(distance.y, self.height),
        )
    }

    /// Similar to [`toroidal::min_coordinate()`](min_coordinate())` but for a 2D toroidal space.
    pub fn get_toroidal_position(&self, position: Vec2) -> Vec2 {
        Vec2::xy(
            min_coordinate(position.x, self.width),
            min_coordinate(position.y, self.height),
        )
    }

    /// Similar to [`toroidal::min_coordinate()`](min_coordinate())` but for an AABB.
    pub fn get_toroidal_aabb(&self, aabb: &AxisAlignmentBoundingBox) -> AxisAlignmentBoundingBox {
        AxisAlignmentBoundingBox::from_bounds(
            min_coordinate(aabb.left(), self.width),
            min_coordinate(aabb.right(), self.width),
            min_coordinate(aabb.top(), self.height),
            min_coordinate(aabb.bottom(), self.height),
        )
    }

    /// Returns the bounds dimension value as a float vector.
    pub fn dimension(&self) -> Vec2 {
        Vec2::xy(self.width as f32, self.height as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_coordinate() {
        assert_eq!(min_coordinate(12.5, 20), 12.5);
        assert_eq!(min_coordinate(32.5, 20), 12.5);
        assert_eq!(min_coordinate(52.5, 20), 12.5);
    }

    #[test]
    fn negative_coordinate() {
        assert_eq!(min_coordinate(-12.5, 20), 7.5);
        assert_eq!(min_coordinate(-32.5, 20), 7.5);
        assert_eq!(min_coordinate(-52.5, 20), 7.5);
    }

    #[test]
    fn positive_distance() {
        assert_eq!(min_distance(7.5, 20), 7.5);
        assert_eq!(min_distance(12.5, 20), -7.5);
        assert_eq!(min_distance(10.0, 20), 10.0);
        assert_eq!(min_distance(20.0, 20), 0.0);
        assert_eq!(min_distance(27.5, 20), 10.0);
    }

    #[test]
    fn negative_distance() {
        assert_eq!(min_distance(-7.5, 20), -7.5);
        assert_eq!(min_distance(-12.5, 20), 7.5);
        assert_eq!(min_distance(-10.0, 20), -10.0);
        assert_eq!(min_distance(-20.0, 20), -0.0);
        assert_eq!(min_distance(-27.5, 20), -10.0);
    }
}
