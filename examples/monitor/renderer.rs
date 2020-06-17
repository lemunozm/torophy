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

pub struct Color {
    data: [f32; 4],
}

impl Color {
    pub fn rgb(red: f32, green: f32, blue: f32) -> Color {
        Color { data:  [red, green, blue, 1.0] }
    }
}

pub struct Renderer{
    display: Display,
    stroke_program: Program,
    perspective: [[f32; 4]; 4],
    dimension: (f32, f32),
}

impl Renderer {
    pub fn new(display: &Display, dimension: (f32, f32)) -> Renderer {
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;
            uniform mat4 perspective;
            uniform mat4 model;
            uniform vec4 color;
            out vec4 v_color;

            void main() {
                v_color = color;
                gl_Position = perspective * model * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec4 v_color;
            out vec4 color;

            void main() {
                color = v_color;
            }
        "#;

        let perspective = [
            [2.0 / dimension.0, 0.0, 0.0, 0.0],
            [0.0, -2.0 / dimension.1, 0.0, 0.0],
            [0.0, 0.0, -2.0, 0.0],
            [-1.0, 1.0, -1.0, 1.0]
        ];

        Renderer {
            display: display.clone(),
            stroke_program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
            perspective,
            dimension,
        }
    }

    pub fn stroke_toroidal_circle(&self, target: &mut Frame, position: (f32, f32), radius: f32, points: usize, color: Color) {
        let step_angle = (2.0 * std::f32::consts::PI) / points as f32;
        let vertexes = (0..points).map(|i| {
            let current_angle = i as f32* step_angle;
            Vertex::new(current_angle.cos() * radius, current_angle.sin() * radius)
        }).collect::<Vec<_>>();

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertexes).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::LineLoop);
        let params = glium::DrawParameters {
            smooth: Some(glium::draw_parameters::Smooth::Fastest),
            .. Default::default()
        };

        let mut draw = |x, y| {
            let model = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, 0.0, 1.0]
            ];

            let uniform_data = uniform!{perspective: self.perspective, model: model, color: color.data};
            target.draw(&vertex_buffer, &indices, &self.stroke_program, &uniform_data, &params).unwrap();
        };

        let mut limits_exceeded = 0;
        draw(position.0, position.1);
        if position.0 - radius < 0.0 {
            draw(position.0 + self.dimension.0, position.1);
            limits_exceeded += 1;
        }
        if position.1 - radius < 0.0 {
            draw(position.0, position.1 + self.dimension.1);
            limits_exceeded += 1;
        }
        if position.0 + radius > self.dimension.0 {
            draw(position.0 - self.dimension.0, position.1);
            limits_exceeded += 1;
        }
        if position.1 + radius > self.dimension.1 {
            draw(position.0, position.1 - self.dimension.1);
            limits_exceeded += 1;
        }
        if limits_exceeded == 2 { // Screen corner case
            draw(position.0 + self.dimension.0, position.1 + self.dimension.1);
            draw(position.0 + self.dimension.0, position.1 - self.dimension.1);
            draw(position.0 - self.dimension.0, position.1 + self.dimension.1);
            draw(position.0 - self.dimension.0, position.1 - self.dimension.1);
        }
    }
}

