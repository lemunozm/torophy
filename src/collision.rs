use super::math::{Vec2, toroidal::{Bounds}};
use super::body::{Body};
use super::shapes::{Shape};

pub struct Contact {
    normal: Vec2,
    overlap: f32,
}

impl Contact {
    pub fn normal(&self) -> Vec2 {
        self.normal
    }

    pub fn overlap(&self) -> f32 {
        self.overlap
    }

    pub fn resolve(&self, b1: &mut Body, b2: &mut Body) {
        self.resolve_overlap(b1, b2);
        self.resolve_velocity(b1, b2);
    }

    fn resolve_overlap(&self, b1: &mut Body, b2: &mut Body) {
        let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
        let displacement = self.normal * (self.overlap / total_inverse_mass);

        b1.displace(displacement * b1.inverse_mass());
        b2.displace(displacement * -b2.inverse_mass());
    }

    fn resolve_velocity(&self, b1: &mut Body, b2: &mut Body) {
        let separating_speed = self.normal * (b1.velocity() - b2.velocity());
        if separating_speed < 0.0 {
            let new_separating_speed = -separating_speed * b1.restitution() * b2.restitution();
            let delta_speed  = new_separating_speed - separating_speed;

            let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
            let impulse = self.normal * (delta_speed / total_inverse_mass);

            b1.add_velocity(impulse * b1.inverse_mass());
            b2.add_velocity(impulse * -b2.inverse_mass());
        }
    }
}

pub struct CollisionResolver<'a> {
    bounds: &'a Bounds,
}

impl<'a> CollisionResolver<'a> {
    pub fn new(bounds: &'a Bounds) -> CollisionResolver {
        CollisionResolver { bounds }
    }

    pub fn test_collision(&self, b1: &Body, b2: &Body) -> Option<Contact> {
        match b1.shape() {
            Shape::None => None,
            Shape::Circle(r1) => match b2.shape() {
                Shape::None => None,
                Shape::Circle(r2) => {
                    self.test_circle_circle(b1.position(), *r1, b2.position(), *r2)
                },
            },
        }
    }

    fn test_circle_circle(&self, p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> Option<Contact> {
        let distance = self.bounds.get_toroidal_distance(p1 - p2);
        let collision_length = r1 + r2;
        if distance.square_length() < collision_length * collision_length {
            let length = distance.square_length().sqrt();
            Some(Contact {
                normal: if length > std::f32::EPSILON { distance / length } else { Vec2::zero() },
                overlap: collision_length - length,
            })
        }
        else { None }
    }
}

