use torophy::{Space, RigidBody, Circle, Vec2, Mobile};

use super::renderer::Renderer;

pub fn draw_space(renderer: &Renderer, target: &mut glium::Frame, space: &Space) {
    for body in space.bodies() {
        let circle = body.shape();
        let radius = circle.radius();
        let position = circle.position();
        let points = (radius * 2.0) as usize;
        renderer.stroke_circle(target, (position.x, position.y), radius, points);
    }
}
