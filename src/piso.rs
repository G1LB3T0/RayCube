use raylib::prelude::*;

pub struct Piso {
    pub y_position: f32,
    pub size: f32,
}

impl Piso {
    pub fn new(y_position: f32, size: f32) -> Self {
        Self { y_position, size }
    }

    pub fn render(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        // Dibujar piso base
        let floor_color = Color::new(40, 40, 45, 255);
        d3.draw_plane(
            Vector3::new(0.0, self.y_position, 0.0), 
            Vector2::new(self.size, self.size), 
            floor_color
        );
        
        // Dibujar cuadrícula
        self.render_grid(d3);
    }

    fn render_grid(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        let grid_size = 10;
        let grid_spacing = self.size / grid_size as f32;
        let grid_color = Color::new(60, 60, 70, 100);
        
        for i in 0..=grid_size {
            let pos = -self.size/2.0 + i as f32 * grid_spacing;
            
            // Líneas horizontales
            d3.draw_line_3D(
                Vector3::new(-self.size/2.0, self.y_position + 0.001, pos),
                Vector3::new(self.size/2.0, self.y_position + 0.001, pos),
                grid_color
            );
            
            // Líneas verticales
            d3.draw_line_3D(
                Vector3::new(pos, self.y_position + 0.001, -self.size/2.0),
                Vector3::new(pos, self.y_position + 0.001, self.size/2.0),
                grid_color
            );
        }
    }
}
