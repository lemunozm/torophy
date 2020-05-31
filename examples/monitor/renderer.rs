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
    pub fn new(display: &Display) -> Renderer {
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

        let perspective = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];

        Renderer {
            display: display.clone(),
            stroke_program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            perspective,
        }
    }

    pub fn stroke_circle(&self, target: &mut Frame, position: (f32, f32), radius: f32, points: usize) {
        let vertex1 = Vertex::new(-5.0, -5.0);
        let vertex2 = Vertex::new(-0.0, 5.0);
        let vertex3 = Vertex::new(5.0, -2.0);
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        target.draw(&vertex_buffer, &indices, &self.stroke_program, &uniform!{perspective: self.perspective}, &Default::default()).unwrap();
    }
}

