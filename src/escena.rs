use raylib::prelude::*;
use crate::{cubo::Cubo, piso::Piso, light::Light};

pub struct Escena {
    pub cubo: Cubo,
    pub piso: Piso,
    pub background_color: Color,
}

impl Escena {
    pub fn new() -> Self {
        let floor_y = -1.5;
        let floor_size = 8.0;
        let cube_size = 1.0;
        
        Self {
            cubo: Cubo::new(cube_size, floor_y),
            piso: Piso::new(floor_y, floor_size),
            background_color: Color::new(20, 25, 35, 255),
        }
    }

    pub fn render(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>, 
                 light: &Light, cam: &Camera3D, show_normals: bool, show_wireframe: bool) {
        // Renderizar piso
        self.piso.render(d3);
        
        // Renderizar luz
        light.render(d3);
        
        // Renderizar cubo base sólido
        self.cubo.render_solid_base(d3);
        
        // Renderizar caras iluminadas
        self.cubo.render_lit_faces(d3, light, cam, show_normals);
        
        // Renderizar sombras (mejorado para evitar parpadeo)
        self.cubo.render_shadow_stable(d3, light, self.piso.y_position);
        
        // Renderizar wireframe solo si está habilitado
        if show_wireframe {
            self.cubo.render_wireframe(d3, cam);
        }
    }

    pub fn get_cube_center_y(&self) -> f32 {
        self.cubo.position_offset
    }
}
