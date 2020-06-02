use glium::{implement_vertex, uniform, Display, Program, Frame, Surface};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { position: [x, y] }
    }
}


pub struct Renderer{
    display: Display,
    stroke_program: Program,
    perspective: [[f32; 4]; 4],
}

impl Renderer {
    pub fn new(display: &Display, dimension: (f32, f32)) -> Renderer {
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 perspective;

            void main() {
                gl_Position = perspective * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let left = 0.0;
        let right = dimension.0;
        let bottom = dimension.1;
        let top = 0.0;
        let near = 0.0;
        let far = 1.0;
        let perspective = [
            [2.0 / right, 0.0, 0.0, 0.0],
            [0.0, -2.0 / bottom, 0.0, 0.0],
            [0.0, 0.0, -2.0, 0.0],
            [-1.0, 1.0, -1.0, 1.0]
        ];

        Renderer {
            display: display.clone(),
            stroke_program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            perspective,
        }
    }

    pub fn stroke_circle(&self, target: &mut Frame, position: (f32, f32), radius: f32, points: usize) {
        let step_angle = (2.0 * std::f32::consts::PI) / points as f32;
        let vertexes = (0..points).map(|i| {
            let current_angle = i as f32* step_angle;
            Vertex::new(position.0 + current_angle.cos() * radius, position.1 + current_angle.sin() * radius)
        }).collect::<Vec<_>>();

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertexes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        let params = glium::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Fastest),
            .. Default::default()
        };

        target.draw(&vertex_buffer, &indices, &self.stroke_program, &uniform!{perspective: self.perspective}, &params).unwrap();
    }
}

