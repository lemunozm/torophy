use super::math::vec2::Vec2;
use super::spatial::Mobile;
use super::shapes::Circle;

pub struct Body {
    shape: Circle, // Should be generic Shape in the future
    velocity: Vec2,
}

impl Body {
    pub fn new(shape: Circle) -> Body {
        Body { shape, velocity: Vec2::zero() }
    }

    pub fn center_of_mass(&self) -> Vec2 {
        self.shape.position()
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }
}

impl Mobile for Body {
    fn set_position(&mut self, position: Vec2) {
        self.shape.set_position(position);
    }

    fn displace(&mut self, displacement: Vec2) {
        self.shape.displace(displacement);
    }

    fn position(&self) -> Vec2 {
        self.shape.position()
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.shape.set_rotation(rotation);
    }

    fn rotate(&mut self, angle: f32) {
        self.shape.rotate(angle);
    }

    fn rotation(&self) -> f32 {
        self.shape.rotation()
    }
}
