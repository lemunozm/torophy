use super::math::vec2::Vec2;
use super::spatial::Mobile;
use super::shapes::Circle;

pub struct RigidBody {
    shape: Circle, // Should be generic Shape in the future
    inverse_mass: f32,
    velocity: Vec2,
    force: Vec2,
    drag_force: (f32, f32),
    restitution: f32,
}

impl RigidBody {
    pub fn new(shape: Circle) -> RigidBody {
        RigidBody {
            shape,
            inverse_mass: 1.0,
            velocity: Vec2::zero(),
            force: Vec2::zero(),
            drag_force: (0.0, 0.0),
            restitution: 1.0,
        }
    }

    pub fn shape(&self) -> &Circle {
        &self.shape
    }

    pub fn center_of_mass(&self) -> Vec2 {
        self.shape.position()
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.inverse_mass = 1.0 / mass;
    }

    pub fn add_mass(&mut self, mass: f32) {
        self.inverse_mass = 1.0 / (1.0 / self.inverse_mass + mass);
    }

    pub fn mass(&self) -> f32 {
        1.0 / self.inverse_mass
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity
    }

    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity
    }

    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_force(&mut self, force: Vec2) {
        self.force = force;
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.force += force;
    }

    pub fn force(&self) -> Vec2 {
        self.force
    }

    pub fn set_drag_force(&mut self, drag_force: (f32, f32)) {
        self.drag_force = drag_force;
    }

    pub fn drag_force(&self) -> (f32, f32) {
        self.drag_force
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn restitution(&self) -> f32 {
        self.restitution
    }
}

impl Mobile for RigidBody {
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
