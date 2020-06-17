use super::math::toroidal::{Bounds};
use super::body::{Particle, Body};
use super::collision::{CollisionResolver, Contact};
use super::util::{BorrowMutTwo};

use std::time::Duration;

pub struct Space {
    bounds: Bounds,
    bodies: Vec<Body>,
    contacts_info: Vec<ContactInfo>, // stored for performance
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            bounds: Bounds::new(width, height),
            bodies: Vec::new(),
            contacts_info: Vec::new(),
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
        self.contacts_info.clear();
        for i1 in 0..self.bodies.len() {
            for i2 in (i1 + 1)..self.bodies.len() {
                if let Some(contact) = collision_resolver.test_collision(&self.bodies[i1], &self.bodies[i2]) {
                    let contact = ContactInfo { first: i1, second: i2, contact };
                    self.contacts_info.push(contact);
                }
            }
        }

        for contact_info in &mut self.contacts_info {
            let (b1, b2) = self.bodies.get_two_mut(contact_info.first, contact_info.second);
            contact_info.contact.resolve(b1, b2);
        }

        for body in &mut self.bodies {
            body.set_position(self.bounds.get_toroidal_position(body.position()));
        }
    }
}

pub struct ContactInfo {
    first: usize,
    second: usize,
    contact: Contact,
}

