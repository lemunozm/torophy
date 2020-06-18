use super::math::vec2::Vec2;
use super::shapes::{Shape, Contact};

pub trait Particle {
    fn integrate(&mut self, dt: f32);
}

pub trait ContactResolver {
    fn resolve_overlap(&self, b1: &mut Body, b2: &mut Body);
    fn resolve_velocity(&self, b1: &mut Body, b2: &mut Body);
}

pub struct Body {
    shape: Option<Shape>,
    position: Vec2,
    inverse_mass: f32,
    velocity: Vec2,
    force: Vec2,
    drag_force: (f32, f32),
    restitution: f32,
}

impl Body {
    pub fn new(position: Vec2) -> Body {
        Body {
            shape: None,
            position,
            inverse_mass: 1.0,
            velocity: Vec2::zero(),
            force: Vec2::zero(),
            drag_force: (0.0, 0.0),
            restitution: 1.0,
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn displace(&mut self, displacement: Vec2) {
        self.position += displacement;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = Some(shape)
    }

    pub fn remove_shape(&mut self) {
        self.shape = None;
    }

    pub fn shape(&self) -> &Option<Shape> {
        &self.shape
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

    pub fn inverse_mass(&self) -> f32 {
        self.inverse_mass
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

impl Particle for Body {
    fn integrate(&mut self, dt: f32) {
        let real_force = self.force - self.velocity * (self.drag_force.0 + self.drag_force.1 * self.velocity.length());
        self.velocity += real_force * (self.inverse_mass * dt);

        self.displace(self.velocity * dt);
        self.force = Vec2::zero();
    }
}

impl ContactResolver for Contact {
    fn resolve_overlap(&self, b1: &mut Body, b2: &mut Body) {
        let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
        let displacement = self.normal() * (self.overlap() / total_inverse_mass);

        b1.displace(displacement * b1.inverse_mass());
        b2.displace(displacement * -b2.inverse_mass());
    }

    fn resolve_velocity(&self, b1: &mut Body, b2: &mut Body) {
        let separating_speed = self.normal() * (b1.velocity() - b2.velocity());
        if separating_speed < 0.0 {
            let new_separating_speed = -separating_speed * b1.restitution() * b2.restitution();
            let delta_speed  = new_separating_speed - separating_speed;

            let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
            let impulse = self.normal() * (delta_speed / total_inverse_mass);

            b1.add_velocity(impulse * b1.inverse_mass());
            b2.add_velocity(impulse * -b2.inverse_mass());
        }
    }
}
