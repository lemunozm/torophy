use super::collision::{CollisionInfo};
use super::body::{Body};

pub struct Contact {
    pub first: usize, //index of bodies list
    pub second: usize, //index of bodies list
    pub collision_info: CollisionInfo,
}

impl Contact {
    pub fn resolve(&self, b1: &mut Body, b2: &mut Body) {
        self.resolve_overlap(b1, b2);
        self.resolve_velocity(b1, b2);
    }

    fn resolve_overlap(&self, b1: &mut Body, b2: &mut Body) {
        let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
        let displacement = self.collision_info.normal * (self.collision_info.overlap / total_inverse_mass);

        b1.displace(displacement * b1.inverse_mass());
        b2.displace(displacement * -b2.inverse_mass());
    }

    fn resolve_velocity(&self, b1: &mut Body, b2: &mut Body) {
        let separating_speed = self.collision_info.normal * (b1.velocity() - b2.velocity());
        if separating_speed < 0.0 {
            let new_separating_speed = -separating_speed * b1.restitution() * b2.restitution();
            let delta_speed  = new_separating_speed - separating_speed;

            let total_inverse_mass = b1.inverse_mass() + b2.inverse_mass();
            let impulse = self.collision_info.normal * (delta_speed / total_inverse_mass);

            b1.add_velocity(impulse * b1.inverse_mass());
            b2.add_velocity(impulse * -b2.inverse_mass());
        }
    }
}

