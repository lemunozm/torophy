use super::math::toroidal;
use super::body::{Particle, Body};

use std::time::Duration;

pub struct Space {
    bounds: toroidal::Bounds,
    bodies: Vec<Body>,
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            bounds: toroidal::Bounds::new(width, height),
            bodies: Vec::new(),
        }
    }

    pub fn bounds(&self) -> &toroidal::Bounds {
        &self.bounds
    }

    pub fn add(&mut self, mut body: Body) {
        body.set_position(self.bounds.get_toroidal_position(body.position()));
        self.bodies.push(body);
    }

    pub fn update(&mut self, duration: Duration) {
        let dt = duration.as_secs_f32();
        for body in &mut self.bodies {
            body.integrate(dt);
        }
        for body in &mut self.bodies {
            body.set_position(self.bounds.get_toroidal_position(body.position()));
        }
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
    }
}
