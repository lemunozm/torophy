use super::math::vec2::Vec2;

pub trait Mobile {
    fn set_position(&mut self, position: Vec2);
    fn displace(&mut self, displacement: Vec2);
    fn position(&self) -> Vec2;

    fn set_rotation(&mut self, rotation: f32);
    fn rotate(&mut self, angle: f32);
    fn rotation(&self) -> f32;
}
