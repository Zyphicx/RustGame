#[macro_use]
extern crate glium;

mod teapot;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let dimensions = glutin::dpi::LogicalSize::new(800.0, 800.0);
    let window = glutin::WindowBuilder::new().with_dimensions(dimensions);
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    //let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness),  1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
        .expect("Shader programs failed to compile");

    let mut t: f32 = 0.0;

    let mut closed = false;
    while !closed {
        t += 0.00002;

        //if t > 2.0 {
        //    t = 0.0;
        //}

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [/*t*/ 0.0, 0.0, 0.0, 100.0f32],
            ],
            u_light: [-1.0, 0.4, 0.9f32],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = display.draw(); //Returns frame object onto which we will draw
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0),1.0); //Change the clear colour
        target.draw((&positions, &normals), &indices, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap(); //Draw the frame onto the screen

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::CloseRequested => {
                        closed = true;
                        println!("Closing window!!!");
                    },
                    _ => (),
                }
                _ => (),
            }
        });
    }
}
