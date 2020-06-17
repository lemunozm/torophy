use torophy::{Space, Body, Shape::Circle, Vec2};
use std::time::Duration;

mod monitor;

fn main() {
    let mut space = Space::new(800, 600);

    let mut body1 = Body::new(Vec2::xy(200.0, 200.0));
    body1.set_shape(Circle(60.0));
    body1.set_mass(3.0);
    body1.set_velocity(Vec2::xy(100.0, 80.0));
    space.add(body1);

    let mut body2 = Body::new(Vec2::xy(400.0, 200.0));
    body2.set_shape(Circle(40.0));
    body2.set_mass(1.0);
    body2.set_velocity(Vec2::xy(-60.0, -200.0));
    space.add(body2);

    let mut body3 = Body::new(Vec2::xy(800.0, 0.0));
    body3.set_shape(Circle(100.0));
    body3.set_mass(3.0);
    body3.set_velocity(Vec2::xy(-100.0, 160.0));
    space.add(body3);

    monitor::GraphicMonitor::new(space)
        .main_loop(|space: &mut Space| {
            space.update(Duration::from_secs_f32(0.0166)); //Real time simulation: 1 / 60 frames
        }
    );
}

