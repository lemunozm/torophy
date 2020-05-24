use super::math::toroidal;
use super::body::RigidBody;

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

    pub fn add(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, interval: Duration) {
        //TODO
    }
}
