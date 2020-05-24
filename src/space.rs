use super::math::toroidal;
use super::body::Body;

use std::time::Duration;

pub struct Space {
    dimension: toroidal::Dimension,
    bodies: Vec<Body>,
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            dimension: toroidal::Dimension {width, height},
            bodies: Vec::new(),
        }
    }

    pub fn add(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&self, interval: Duration) {
        //TODO
    }
}
