use nalgebra::{vector, Matrix4, Point3, Vector3};
use winit::event::VirtualKeyCode as Keycode;

type KeySet = std::collections::HashSet<Keycode>;

pub trait Camera {
    fn view_matrix(&self) -> Matrix4<f32>;
}

pub struct FPSCamera {
    pos: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
    speed: f32,
}

impl FPSCamera {
    pub fn new(pos: Point3<f32>, sensitivity: f32, speed: f32) -> FPSCamera {
        FPSCamera {
            pos,
            front: vector![0.0, 0.0, 1.0],
            up: vector![0.0, 1.0, 0.0],
            yaw: 0.0,
            pitch: 0.0,
            sensitivity,
            speed,
        }
    }

    pub fn update(&mut self, keys: &KeySet, dt: std::time::Duration) {
        // Delta time seconds, for ease of use in calculations
        let dt = dt.as_secs_f32();

        // Calculate normalized front from yaw and pitch
        self.front.x = self.yaw.sin();
        self.front.y = self.pitch.sin();
        self.front.z = self.yaw.cos();
        self.front.normalize_mut();

        // Normalized camera right vector
        let right = self.up.cross(&self.front).normalize();

        // Move forward/back
        if keys.contains(&Keycode::W) {
            self.pos -= self.front * dt * self.speed;
        }
        if keys.contains(&Keycode::S) {
            self.pos += self.front * dt * self.speed;
        }

        // Strafe
        if keys.contains(&Keycode::D) {
            self.pos += right * dt * self.speed;
        }
        if keys.contains(&Keycode::A) {
            self.pos -= right * dt * self.speed;
        }
    }

    pub fn mouse_moved(&mut self, delta: (f64, f64)) {
        self.yaw -= delta.0 as f32 * self.sensitivity;
        self.pitch += delta.1 as f32 * self.sensitivity;

        if self.pitch > std::f32::consts::FRAC_PI_2 {
            self.pitch = std::f32::consts::FRAC_PI_2;
        }
        if self.pitch < -std::f32::consts::FRAC_PI_2 {
            self.pitch = -std::f32::consts::FRAC_PI_2;
        }
    }
}

impl Camera for FPSCamera {
    fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_lh(&self.pos, &(self.pos + self.front), &self.up)
    }
}
