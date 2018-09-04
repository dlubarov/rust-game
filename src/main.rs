extern crate glium;

mod body;
mod camera;
mod character;
mod cylinder;
mod human;
mod mat3;
mod mat4;
mod mesh;
mod player;
mod vec2;
mod vec3;
mod vec4;
mod vertex;

use std::time::Duration;
use std::collections::HashSet;
use std::thread;
use glium::glutin;
use glium::Surface;
use glutin::Event;
use glutin::WindowEvent;
use glutin::VirtualKeyCode;
use glutin::ElementState;

use body::*;
use camera::*;
use human::*;
use player::*;
use vec2::*;
use vec3::*;

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
    // TODO uncomment
    let context = glutin::ContextBuilder::new();//.with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let mut player = Player { position: Vec2f { x: 0.0, y: 0.0 }, direction: 0.0 };
    let mut npc_body = Human::new(&display);

    let mut pressed_keys = HashSet::new();

    let mut closed = false;
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => closed = true,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            let pressed = input.state == ElementState::Pressed;
                            if pressed {
                                pressed_keys.insert(key);
                            } else {
                                pressed_keys.remove(&key);
                            }

                            if key == VirtualKeyCode::Escape {
                                closed = true;
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        });

        player.physics(&pressed_keys);

        // Temporary controls for moving the NPC around.
        if pressed_keys.contains(&VirtualKeyCode::T) {
            npc_body.forward();
        }

        let mut target = display.draw();
        let pos = Vec3f { x: player.position.x, y: 0.0, z: player.position.y };
        let view = view_matrix(&pos, player.direction);
        let (width, height) = target.get_dimensions();
        let projection = perspective_matrix(width as f32, height as f32);

        target.clear_color_and_depth((1.0, 0.9, 0.9, 1.0), 1.0);
        npc_body.draw(&mut target, &program, &view, &projection);
        target.finish().unwrap();

        thread::sleep(Duration::from_millis(10))
    }
}
