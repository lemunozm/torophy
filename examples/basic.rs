use torophy::{Space, RigidBody, Shape::Circle, Vec2};
use std::time::Duration;

mod monitor;

fn main() {
    let mut body1 = RigidBody::new(Vec2::xy(200.0, 200.0));
    body1.set_shape(Circle(30.0));
    body1.set_mass(3.0);
    body1.set_velocity(Vec2::xy(70.0, 40.0));

    let mut body2 = RigidBody::new(Vec2::xy(400.0, 200.0));
    body2.set_shape(Circle(20.0));
    body2.set_mass(1.0);
    body2.set_velocity(Vec2::xy(-30.0, -100.0));

    let mut body3 = RigidBody::new(Vec2::xy(800.0, 0.0));
    body3.set_shape(Circle(50.0));
    body3.set_mass(3.0);
    body3.set_velocity(Vec2::xy(50.0, 80.0));

    let mut space = Space::new(800, 600);
    space.add(body1);
    space.add(body2);
    space.add(body3);

    monitor::GraphicMonitor::new(space)
        .main_loop(|space: &mut Space| {
            space.update(Duration::from_secs_f32(0.0166));
        }
    );
}

