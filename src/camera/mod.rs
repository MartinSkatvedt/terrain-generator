use glutin::event::VirtualKeyCode;

pub struct Camera {
    pub position: glm::Vec3,
    pub direction: glm::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub front: glm::Vec3,
    pub up: glm::Vec3,
    pub move_speed: f32,
    pub camera_sensitivity: f32,
}

impl Camera {
    pub fn new() -> Self {
        let camera = Camera {
            position: glm::vec3(0.0, 4.0, 5.0),
            direction: glm::vec3(1.0, 0.0, 1.0),
            yaw: -90.0,
            pitch: 0.0,
            front: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, 1.0, 0.0),
            move_speed: 150.0,
            camera_sensitivity: 100.0,
        };

        camera
    }

    pub fn update_camera_vectors(&mut self) {
        self.direction.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.direction.y = self.pitch.to_radians().sin();
        self.direction.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = glm::normalize(&self.direction);
    }

    pub fn get_look_at_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.front), &self.up)
    }

    pub fn handle_key_input(&mut self, key: VirtualKeyCode, delta_time: f32) {
        match key {
            VirtualKeyCode::D => {
                self.position += self.move_speed
                    * delta_time
                    * glm::normalize(&glm::cross(&self.front, &self.up));
            }
            VirtualKeyCode::A => {
                self.position -= self.move_speed
                    * delta_time
                    * glm::normalize(&glm::cross(&self.front, &self.up));
            }
            VirtualKeyCode::Space => {
                self.position += self.move_speed * delta_time * self.up;
            }
            VirtualKeyCode::LShift => {
                self.position -= self.move_speed * delta_time * self.up;
            }
            VirtualKeyCode::W => {
                self.position += self.move_speed * delta_time * self.front;
            }
            VirtualKeyCode::S => {
                self.position -= self.move_speed * delta_time * self.front;
            }
            VirtualKeyCode::Up => {
                self.pitch += delta_time * self.camera_sensitivity;
            }
            VirtualKeyCode::Down => {
                self.pitch -= delta_time * self.camera_sensitivity;
            }
            VirtualKeyCode::Left => {
                self.yaw -= delta_time * self.camera_sensitivity;
            }
            VirtualKeyCode::Right => {
                self.yaw += delta_time * self.camera_sensitivity;
            }
            // default handler:
            _ => {}
        }
    }
}
