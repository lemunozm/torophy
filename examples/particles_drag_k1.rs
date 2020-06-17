use torophy::{Space, Body, Vec2};
use std::time::Duration;
use rand::{Rng, SeedableRng, rngs::StdRng, distributions::Uniform};

mod monitor;

fn main() {
    let mut space = Space::new(800, 600);

    let angle_dist = Uniform::new(-std::f32::consts::PI, std::f32::consts::PI);
    let speed_dist = Uniform::new(5.0, 100.0);
    let drag_force_k1 = 0.5;
    let mut rng = StdRng::seed_from_u64(0);

    for _ in 0..1000 {
        let mut body = Body::new(space.bounds().dimension() / 2.0);
        body.set_velocity(Vec2::from_angle(rng.sample(angle_dist)) * rng.sample(speed_dist));
        body.set_drag_force((drag_force_k1, 0.0));
        space.add(body);
    }

    monitor::GraphicMonitor::new(space)
        .main_loop(|space: &mut Space| {
            space.update(Duration::from_secs_f32(0.0166)); //Real time simulation: 1 / 60 frames
        }
    );
}

