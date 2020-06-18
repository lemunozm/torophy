use super::math::{Vec2, toroidal::{Bounds}};

#[derive(Clone)]
pub enum Shape {
    Circle(f32),
}

impl Shape {
    pub fn half_dimension(&self) -> Vec2 {
        match *self {
            Shape::Circle(radius) => Vec2::xy(radius, radius)
        }
    }
}

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
}


pub struct CollisionResolver<'a> {
    bounds: &'a Bounds,
}

impl<'a> CollisionResolver<'a> {
    pub fn new(bounds: &'a Bounds) -> CollisionResolver {
        CollisionResolver { bounds }
    }

    pub fn check_collision(&self, p1: Vec2, s1: &Shape, p2: Vec2, s2: &Shape) -> Option<Contact> {
        match *s1 {
            Shape::Circle(r1) => match *s2 {
                Shape::Circle(r2) => {
                    self.check_circle_circle(p1, r1, p2, r2)
                },
            },
        }
    }

    fn check_circle_circle(&self, p1: Vec2, r1: f32, p2: Vec2, r2: f32) -> Option<Contact> {
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
