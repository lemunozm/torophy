use torophy::{Space, RigidBody, Circle, Vec2};

use super::renderer::Renderer;

pub fn draw_space(renderer: &Renderer, target: &mut glium::Frame, space: &Space) {
    renderer.stroke_circle(target, (200.0, 100.0), 20.0, 10);
}
