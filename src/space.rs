use super::math::toroidal;
use super::body::{Particle, RigidBody};

use std::time::Duration;

pub struct Space {
    dimension: toroidal::Dimension,
    bodies: Vec<RigidBody>,
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            dimension: toroidal::Dimension {width, height},
            bodies: Vec::new(),
        }
    }

    pub fn dimension(&self) -> &toroidal::Dimension {
        &self.dimension
    }

    pub fn add(&mut self, mut body: RigidBody) {
        body.set_position(self.dimension.get_toroidal_position(body.position()));
        self.bodies.push(body);
    }

    pub fn update(&mut self, duration: Duration) {
        let dt = duration.as_secs_f32();
        for body in &mut self.bodies {
            body.integrate(dt);
        }
        for body in &mut self.bodies {
            body.set_position(self.dimension.get_toroidal_position(body.position()));
        }
    }

    pub fn bodies(&self) -> &Vec<RigidBody> {
        &self.bodies
    }
}
