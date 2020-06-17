use super::math::{Vec2, toroidal::{Bounds}};
use super::body::{Body};
use super::shapes::{Shape};


pub struct CollisionInfo {
    pub normal: Vec2,
    pub overlap: f32,
}

pub struct CollisionResolver<'a> {
    bounds: &'a Bounds,
}

impl<'a> CollisionResolver<'a> {
    pub fn new(bounds: &'a Bounds) -> CollisionResolver {
        CollisionResolver { bounds }
    }

    pub fn test_collision(&self, b1: &Body, b2: &Body) -> Option<CollisionInfo> {
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

    fn test_circle_circle(&self, p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> Option<CollisionInfo> {
        let distance = self.bounds.get_toroidal_distance(p1 - p2);
        let collision_length = r1 + r2;
        if distance.square_length() < collision_length * collision_length {
            let length = distance.square_length().sqrt();
            Some(CollisionInfo {
                normal: if length > std::f32::EPSILON { distance / length } else { Vec2::zero() },
                overlap: collision_length - length,
            })
        }
        else { None }
    }
}

