use std::collections::HashSet;
use glutin::VirtualKeyCode;

use vec2::*;

const SPEED: f32 = 0.1;

pub struct Player {
    pub position: Vec2f,
    pub direction: f32,
}

impl Player {
    pub fn physics(&mut self, pressed_keys: &HashSet<VirtualKeyCode>) {
        let forward = pressed_keys.contains(&VirtualKeyCode::Up) || pressed_keys.contains(&VirtualKeyCode::W);
        let left = pressed_keys.contains(&VirtualKeyCode::Left) || pressed_keys.contains(&VirtualKeyCode::A);
        let back = pressed_keys.contains(&VirtualKeyCode::Down) || pressed_keys.contains(&VirtualKeyCode::S);
        let right = pressed_keys.contains(&VirtualKeyCode::Right) || pressed_keys.contains(&VirtualKeyCode::D);

        let mut step_dir = Vec2f::ZERO;
        if forward ^ back {
            let forward_dir = Vec2f { x: self.direction.cos(), y: -self.direction.sin() };
            step_dir += forward_dir * (forward as i8 - back as i8) as f32;
        }
        if left ^ right {
            let left_dir = Vec2f { x: -self.direction.sin(), y: -self.direction.cos() };
            step_dir += left_dir * (left as i8 - right as i8) as f32;
        }

        let displacement = match step_dir {
            d if d == Vec2f::ZERO => Vec2f::ZERO,
            _ => step_dir.normalized() * SPEED,
        };
        self.position += displacement;
    }
}
