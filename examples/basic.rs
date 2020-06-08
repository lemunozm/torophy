use torophy::{Space, RigidBody, Circle, Vec2};
use std::time::Duration;

mod monitor;

fn main() {
    let shape = Circle::new(Vec2::xy(200.0, 200.0), 30.0);
    let mut body1 = RigidBody::new(shape);
    body1.set_mass(3.0);
    body1.set_velocity(Vec2::xy(7.0, 4.0));

    let shape = Circle::new(Vec2::xy(400.0, 200.0), 20.0);
    let mut body2 = RigidBody::new(shape);
    body2.set_mass(1.0);
    body2.set_velocity(Vec2::xy(-3.0, -1.0));

    let shape = Circle::new(Vec2::xy(800.0, 0.0), 50.0);
    let mut body3 = RigidBody::new(shape);
    body3.set_mass(3.0);
    body3.set_velocity(Vec2::xy(7.0, 4.0));

    let mut space = Space::new(800, 600);
    space.add(body1);
    space.add(body2);
    space.add(body3);

    monitor::GraphicMonitor::new(space)
        .main_loop(|space: &mut Space| {
            space.update(Duration::from_millis(1));
        }
    );
}

