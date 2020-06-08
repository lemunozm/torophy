use super::math::vec2::Vec2;
use super::spatial::Mobile;

pub trait AxisAlignmentBoundingBox {
    fn half_dimension(&self) -> Vec2;
}

#[derive(Clone)]
pub struct Circle {
    position: Vec2, //Really used when the position could differ from the body's center of mass;
    rotation: f32,
    radius: f32,
}

impl Circle {
    pub fn new(position: Vec2, radius: f32) -> Circle {
        Circle { position, rotation: 0.0, radius }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Mobile for Circle {
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn displace(&mut self, displacement: Vec2) {
        self.position += displacement;
    }

    fn position(&self) -> Vec2 {
        self.position
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    fn rotation(&self) -> f32 {
        self.rotation
    }
}

impl AxisAlignmentBoundingBox for Circle {
    fn half_dimension(&self) -> Vec2 {
        Vec2::xy(self.radius, self.radius)
    }
}
