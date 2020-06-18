use super::math::toroidal::{Bounds};
use super::body::{Body, Particle, ContactResolver};
use super::shapes::{CollisionResolver, Contact};
use super::spatial_table::{SpatialTable};
use super::util::{BorrowMutTwo};

use std::time::Duration;
use std::slice::{Iter, IterMut};

pub struct Space {
    bounds: Bounds,
    bodies: Vec<Body>,
    spatial_table: SpatialTable,
    contacts_info: Vec<ContactInfo>, // stored for performance
}

impl Space {
    pub fn new(width: u32, height: u32) -> Space {
        Space {
            bounds: Bounds::new(width, height),
            bodies: Vec::new(),
            contacts_info: Vec::new(),
            spatial_table: SpatialTable::new(width, height, 30.0),
        }
    }

    pub fn configure_spatial_table(&mut self, cell_size: f32) {
        self.spatial_table = SpatialTable::new(self.bounds.width, self.bounds.height, cell_size);
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn bodies(&self) -> Iter<Body> {
        self.bodies.iter()
    }

    pub fn bodies_mut(&mut self) -> IterMut<Body> {
        self.bodies.iter_mut()
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

        self.spatial_table.clear();
        for (i, body) in self.bodies.iter().enumerate() {
            if let Some(aabb) = body.aabb() {
                self.spatial_table.insert(i, &self.bounds.get_toroidal_aabb(&aabb));
            }
        }

        let collision_resolver = CollisionResolver::new(&self.bounds);
        self.contacts_info.clear();
        for pair in self.spatial_table.pairs() {
            let b1 = &self.bodies[pair.0];
            let b2 = &self.bodies[pair.1];
            let s1 = b1.shape().unwrap();
            let s2 = b2.shape().unwrap();
            if let Some(contact) = collision_resolver.check_collision(b1.position(), &s1, b2.position(), &s2) {
                let contact = ContactInfo { first: pair.0, second: pair.1, contact };
                self.contacts_info.push(contact);
            }
        }

        //TODO: brute force ---
        /*
        let collision_resolver = CollisionResolver::new(&self.bounds);
        self.contacts_info.clear();
        for i1 in 0..self.bodies.len() {
            let b1 = &self.bodies[i1];
            if let Some(s1) = b1.shape() {
                for i2 in (i1 + 1)..self.bodies.len() {
                    let b2 = &self.bodies[i2];
                    if let Some(s2) = b2.shape() {
                        if let Some(contact) = collision_resolver.check_collision(b1.position(), &s1, b2.position(), &s2) {
                            let contact = ContactInfo { first: i1, second: i2, contact };
                            self.contacts_info.push(contact);
                        }
                    }
                }
            }
        }
        */
        // ---

        for ContactInfo { first, second, contact } in &mut self.contacts_info {
            let (b1, b2) = self.bodies.get_two_mut(*first, *second);
            contact.resolve_overlap(b1, b2);
            contact.resolve_velocity(b1, b2);
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

