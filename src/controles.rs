use raylib::prelude::*;

pub struct CameraControls {
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl CameraControls {
    pub fn new(initial_position: Vector3, target: Vector3) -> Self {
        let offset = initial_position - target;
        let radius = (offset.x * offset.x + offset.y * offset.y + offset.z * offset.z).sqrt();
        let yaw = offset.z.atan2(offset.x);
        let pitch = (offset.y / radius).asin();
        
        Self { radius, yaw, pitch }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        // Controles con teclado
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) { 
            self.yaw -= 0.02; 
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) { 
            self.yaw += 0.02; 
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) || rl.is_key_down(KeyboardKey::KEY_W) { 
            self.pitch = (self.pitch + 0.02).clamp(-1.4, 1.4); 
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) || rl.is_key_down(KeyboardKey::KEY_S) { 
            self.pitch = (self.pitch - 0.02).clamp(-1.4, 1.4); 
        }
        if rl.is_key_down(KeyboardKey::KEY_Q) { 
            self.radius = (self.radius - 0.08).clamp(2.0, 30.0); 
        }
        if rl.is_key_down(KeyboardKey::KEY_E) { 
            self.radius = (self.radius + 0.08).clamp(2.0, 30.0); 
        }

        // Control con mouse
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_delta = rl.get_mouse_delta();
            self.yaw += mouse_delta.x * 0.005;
            self.pitch = (self.pitch - mouse_delta.y * 0.005).clamp(-1.4, 1.4);
        }

        // Zoom con rueda del mouse
        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            self.radius = (self.radius - wheel_move * 0.5).clamp(2.0, 30.0);
        }
    }

    pub fn update_camera_position(&self, cam: &mut Camera3D) {
        let cp = self.pitch.cos();
        cam.position = Vector3::new(
            self.radius * self.yaw.cos() * cp, 
            self.radius * self.pitch.sin(), 
            self.radius * self.yaw.sin() * cp
        ) + cam.target;
    }
}

pub struct LightControls {
    pub rotation: f32,
    pub auto_rotate: bool,
    pub radius: f32,
    pub height: f32,
    pub intensity: f32, // Nueva propiedad para controlar intensidad
}

impl LightControls {
    pub fn new() -> Self {
        Self {
            rotation: 0.0,
            auto_rotate: true,
            radius: 5.0,
            height: 4.0,
            intensity: 1.0, // Intensidad por defecto
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_L) { 
            self.auto_rotate = !self.auto_rotate; 
        }

        // Controles para intensidad de luz
        if rl.is_key_down(KeyboardKey::KEY_KP_ADD) || rl.is_key_down(KeyboardKey::KEY_EQUAL) {
            self.intensity = (self.intensity + 0.02).min(3.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_KP_SUBTRACT) || rl.is_key_down(KeyboardKey::KEY_MINUS) {
            self.intensity = (self.intensity - 0.02).max(0.1);
        }

        if self.auto_rotate {
            self.rotation += 0.01;
        }
    }

    pub fn update_light_position(&self, light: &mut crate::light::Light) {
        light.pos = [
            self.radius * self.rotation.cos(),
            self.height,
            self.radius * self.rotation.sin(),
        ];
        // Actualizar intensidad de la luz
        light.color = [self.intensity, self.intensity, self.intensity];
    }
}
