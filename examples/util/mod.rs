use torophy::{Space, RigidBody, Circle, Vec2};

use piston_window::*;

pub struct GraphicMonitor<'a> {
    space: &'a mut Space,
    window: PistonWindow,
}

impl<'a> GraphicMonitor<'a> {
    pub fn new(space: &'a mut Space) -> GraphicMonitor {
        let width = space.dimension().width;
        let height = space.dimension().height;
        GraphicMonitor {
            space,
            window: WindowSettings::new("Torophy monitor", [width, height]) .exit_on_esc(true).build().unwrap(),
        }
    }

    pub fn run_with<F>(&mut self, mut frame_action: F)
    where F: FnMut(&mut Space) {
        loop {
            frame_action(&mut self.space);
        }
    }
}
