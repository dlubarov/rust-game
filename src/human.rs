#[macro_use]

use glium::*;
use glium::index::*;
use body::*;
use vec3::*;
use mat4::*;
use cylinder::*;
use vertex::*;
use mesh::*;
use camera::*;

const NECK_LENGTH: f32 = 0.2;
const TORSO_LENGTH: f32 = 0.8;
const TORSO_WIDTH: f32 = 0.3;
const UPPER_ARM_LENGTH: f32 = 0.3;
const LOWER_ARM_LENGTH: f32 = 0.3;
const UPPER_LEG_LENGTH: f32 = 0.4;
const LOWER_LEG_LENGTH: f32 = 0.4;

pub struct Human {
    pos: Vec3f,
    joints: Joints,
    objects: LimbObjects,
}

#[derive(Default)]
struct Joints {
    left_shoulder: Joint,
    right_shoulder: Joint,
    left_elbow: Joint,
    right_elbow: Joint,
    left_knee: Joint,
    right_knee: Joint,
    left_sacroiliac: Joint,
    right_sacroiliac: Joint,
}

struct LimbObjects {
    left_lower_leg: MeshObjects,
}

impl Human {
    pub fn new(display: &Display) -> Self {
        let objects = LimbObjects {
            left_lower_leg: MeshObjects::create(display, &make_cylinder(0, 0.0, LOWER_LEG_LENGTH, 0.1)),
        };
        Human {
            pos: Vec3f { x: 9.0, y: 9.0, z: 9.0 },
            joints: Default::default(),
            objects,
        }
    }

    pub fn stand(&mut self) {
    }

    pub fn sit(&mut self) {
    }

    pub fn forward(&mut self) {
        //self.joints.left_knee.yaw += 0.03;
    }

    pub fn backward(&mut self) {
    }

    fn left_sacroiliac_transform(&self) -> Mat4f {
        Mat4f::translation(-0.1, -TORSO_LENGTH, 0.0) * self.joints.left_sacroiliac.to_matrix()
    }

    fn right_sacroiliac_transform(&self) -> Mat4f {
        Mat4f::translation(0.1, -TORSO_LENGTH, 0.0) * self.joints.right_sacroiliac.to_matrix()
    }

    fn left_knee_transform(&self) -> Mat4f {
        Mat4f::translation_vec(self.pos)
            * self.left_sacroiliac_transform()
            * Mat4f::translation(0.0, -UPPER_LEG_LENGTH, 0.0)
            * self.joints.left_knee.to_matrix()
    }

    fn right_knee_transform(&self) -> Mat4f {
        self.right_sacroiliac_transform() * Mat4f::translation(0.0, -UPPER_LEG_LENGTH, 0.0) * self.joints.right_knee.to_matrix()
    }

    fn left_shoulder_pos(&self) -> Vec3f {
        self.pos + Vec3f { x : -TORSO_WIDTH, y: -NECK_LENGTH, z: 0.0 }
    }

    fn right_shoulder_pos(&self) -> Vec3f {
        self.pos + Vec3f { x : TORSO_WIDTH, y: -NECK_LENGTH, z: 0.0 }
    }

    fn left_hip_pos(&self) -> Vec3f {
        self.pos + Vec3f { x : -TORSO_WIDTH, y: -NECK_LENGTH - TORSO_LENGTH, z: 0.0 }
    }

    fn right_hip_pos(&self) -> Vec3f {
        self.pos + Vec3f { x : TORSO_WIDTH, y: -NECK_LENGTH - TORSO_LENGTH, z: 0.0 }
    }
}

impl Body for Human {
    fn name() -> &'static str {
        "human"
    }

    fn draw(&self, target: &mut Frame, program: &Program) {
        // Draw lower legs.
        let (width, height) = target.get_dimensions();
        let model = self.left_knee_transform();
        let view = view_matrix(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[0.0, 1.0, 0.0]);
        let uniforms = uniform! {
            modelview: (view * model).values,
            projection: perspective_matrix(width as f32, height as f32).values,
        };
        target.draw(&self.objects.left_lower_leg.vertex_buffer,
                    &self.objects.left_lower_leg.index_buffer,
                    &program, &uniforms, &Default::default()).unwrap();
    }
}
