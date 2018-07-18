extern crate glium;

mod body;
mod camera;
mod character;
mod cylinder;
mod human;
mod mat3;
mod mat4;
mod mesh;
mod vec2;
mod vec3;
mod vec4;
mod vertex;

use std::time::Duration;
use std::thread;
use glium::glutin;
use glium::Surface;

use body::Body;
use human::Human;

const VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec3 position;
    in vec3 normal;

    //out vec3 v_normal;

    uniform mat4 modelview;
    //uniform mat3 transpose_inverse_modelview;
    uniform mat4 projection;

    void main() {
        //mat4 modelview = view * model;
        //v_normal = transpose(inverse(mat3(modelview))) * normal;
        gl_Position = projection * modelview * vec4(position, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    //in vec3 v_normal;

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
        //color = vec4(normalize(v_normal), 1.0);
    }
"#;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions((1024, 768).into())
        .with_title("Hello world");
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let mut player_body = Human::new(&display);

    let mut closed = false;
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        player_body.forward();

        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 0.9, 0.9, 1.0), 1.0);
        player_body.draw(&mut target, &program);
        target.finish().unwrap();

        thread::sleep(Duration::from_millis(10))
    }
}
