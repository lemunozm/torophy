use super::math::toroidal::{Bounds};
use super::body::{Particle, Body};
use super::collision::{CollisionResolver};
use super::contact::{Contact};
use super::util::{BorrowMutTwo};

use std::time::Duration;

pub struct Space {
    bounds: Bounds,
    bodies: Vec<Body>,
    contacts: Vec<Contact>, // stored for performance
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            bounds: Bounds::new(width, height),
            bodies: Vec::new(),
            contacts: Vec::new(),
        }
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn bodies(&self) -> &Vec<Body> {
        &self.bodies
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

        let collision_resolver = CollisionResolver::new(&self.bounds);
        self.contacts.clear();
        for i1 in 0..self.bodies.len() {
            for i2 in (i1 + 1)..self.bodies.len() {
                if let Some(collision_info) = collision_resolver.test_collision(&self.bodies[i1], &self.bodies[i2]) {
                    let contact = Contact {
                        first: i1,
                        second: i2,
                        collision_info,
                    };
                    self.contacts.push(contact);
                }
            }
        }

        for contact in &mut self.contacts {
            let (b1, b2) = self.bodies.get_two_mut(contact.first, contact.second);
            contact.resolve(b1, b2);
        }

        for body in &mut self.bodies {
            body.set_position(self.bounds.get_toroidal_position(body.position()));
        }
    }
}
