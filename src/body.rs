use super::math::vec2::Vec2;
use super::shapes::Shape;

pub trait Particle {
    fn integrate(&mut self, dt: f32);
}

pub struct Body {
    shape: Shape,
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
            shape: Shape::None,
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
        self.shape = shape
    }

    pub fn shape(&self) -> &Shape {
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

