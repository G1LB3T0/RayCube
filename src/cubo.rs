use raylib::prelude::*;
use crate::light;
use crate::colores;

pub struct Cubo {
    pub verts: [Vector3; 8],
    pub faces: [([usize; 3], [usize; 3], Vector3); 6],
    pub size: f32,
    pub position_offset: f32,
}

impl Cubo {
    pub fn new(size: f32, floor_y: f32) -> Self {
        let s = size;
        let cube_y_offset = floor_y + s;
        
        let verts = [
            Vector3::new(-s, cube_y_offset - s, -s), // 0 - base inferior
            Vector3::new( s, cube_y_offset - s, -s), // 1
            Vector3::new( s, cube_y_offset + s, -s), // 2 - base superior
            Vector3::new(-s, cube_y_offset + s, -s), // 3
            Vector3::new(-s, cube_y_offset - s,  s), // 4 - base inferior
            Vector3::new( s, cube_y_offset - s,  s), // 5
            Vector3::new( s, cube_y_offset + s,  s), // 6 - base superior
            Vector3::new(-s, cube_y_offset + s,  s), // 7
        ];

        let faces = [
            // Frente (z+): 4-5-6-7 - orden correcto para CCW desde fuera
            ([4, 6, 5], [4, 7, 6], Vector3::new(0.0, 0.0, 1.0)),
            // Atrás (z-): 0-1-2-3 - orden correcto para CCW desde fuera
            ([0, 1, 2], [0, 2, 3], Vector3::new(0.0, 0.0, -1.0)),
            // Derecha (x+): 1-5-6-2 - orden correcto
            ([1, 6, 2], [1, 5, 6], Vector3::new(1.0, 0.0, 0.0)),
            // Izquierda (x-): 0-4-7-3 - orden correcto
            ([0, 3, 7], [0, 7, 4], Vector3::new(-1.0, 0.0, 0.0)),
            // Arriba (y+): 3-2-6-7 - orden correcto
            ([3, 6, 7], [3, 2, 6], Vector3::new(0.0, 1.0, 0.0)),
            // Abajo (y-): 0-1-5-4 - orden correcto
            ([0, 5, 4], [0, 1, 5], Vector3::new(0.0, -1.0, 0.0)),
        ];

        Self {
            verts,
            faces,
            size: s,
            position_offset: cube_y_offset,
        }
    }

    pub fn render_solid_base(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
        // Cubo base más oscuro para que la iluminación destaque más
        let cube_color = Color::new(
            (colores::OBJECT_COLOR[0] * 120.0) as u8, // Más oscuro que antes
            (colores::OBJECT_COLOR[1] * 120.0) as u8,
            (colores::OBJECT_COLOR[2] * 120.0) as u8,
            255
        );
        d3.draw_cube(Vector3::new(0.0, self.position_offset, 0.0), self.size * 2.0, self.size * 2.0, self.size * 2.0, cube_color);
    }

    pub fn render_lit_faces(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>, 
                           light: &light::Light, cam: &Camera3D, show_normals: bool) {
        // Color base más oscuro para mejor contraste
        let base_color = Color::new(
            (colores::OBJECT_COLOR[0] * 180.0) as u8, // Reducido de 255 a 180
            (colores::OBJECT_COLOR[1] * 180.0) as u8,
            (colores::OBJECT_COLOR[2] * 180.0) as u8,
            255,
        );
        let light_col = light::color_vec(light);

        // Crear lista de triángulos para shadow testing
        let mut all_triangles = Vec::new();
        for (tri1, tri2, _) in self.faces.iter() {
            all_triangles.push((self.verts[tri1[0]], self.verts[tri1[1]], self.verts[tri1[2]]));
            all_triangles.push((self.verts[tri2[0]], self.verts[tri2[1]], self.verts[tri2[2]]));
        }

        unsafe { raylib::ffi::rlDisableBackfaceCulling(); }
        
        for (tri1, tri2, normal_hint) in self.faces.iter() {
            let normal = *normal_hint;
            
            // Primer triángulo
            let (a, b, c) = (self.verts[tri1[0]], self.verts[tri1[1]], self.verts[tri1[2]]);
            let center1 = (a + b + c) / 3.0;
            
            let view_dir = (cam.position - center1).normalized();
            if normal.dot(view_dir) > -0.5 {
                // Usar iluminación con menos luz ambiente y más contraste
                let color1 = light::shade_with_shadows(normal, center1, light, base_color, light_col, colores::AMBIENT_LIGHT, &all_triangles);
                d3.draw_triangle3D(a, b, c, color1);
            }

            // Segundo triángulo
            let (a2, b2, c2) = (self.verts[tri2[0]], self.verts[tri2[1]], self.verts[tri2[2]]);
            let center2 = (a2 + b2 + c2) / 3.0;
            
            let view_dir2 = (cam.position - center2).normalized();
            if normal.dot(view_dir2) > -0.5 {
                let color2 = light::shade_with_shadows(normal, center2, light, base_color, light_col, colores::AMBIENT_LIGHT, &all_triangles);
                d3.draw_triangle3D(a2, b2, c2, color2);
            }

            if show_normals {
                let ctri = (a + b + c) / 3.0;
                d3.draw_line_3D(ctri, ctri + normal * 0.6, Color::YELLOW);
                let ctri2 = (a2 + b2 + c2) / 3.0;
                d3.draw_line_3D(ctri2, ctri2 + normal * 0.6, Color::YELLOW);
            }
        }
        
        unsafe { raylib::ffi::rlEnableBackfaceCulling(); }
    }

    pub fn render_wireframe(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>, cam: &Camera3D) {
        unsafe { 
            raylib::ffi::rlDisableDepthTest(); 
            raylib::ffi::rlSetLineWidth(3.0);
        }
        
        let edges = [
            // Base inferior (z = -1)
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Base superior (z = +1)
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Conexiones verticales
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];
        
        for (i, j) in edges.iter() {
            let p1 = self.verts[*i];
            let p2 = self.verts[*j];
            
            let cam_dir = (cam.position - ((p1 + p2) * 0.5)).normalized();
            let offset = cam_dir * 0.001;
            
            d3.draw_line_3D(p1 + offset, p2 + offset, Color::WHITE);
        }
        
        for &vertex in self.verts.iter() {
            let cam_dir = (cam.position - vertex).normalized();
            let offset = cam_dir * 0.002;
            d3.draw_sphere(vertex + offset, 0.02, Color::YELLOW);
        }
        
        unsafe { 
            raylib::ffi::rlEnableDepthTest();
            raylib::ffi::rlSetLineWidth(1.0);
        }
    }

    // Versión mejorada para evitar parpadeo de sombras
    pub fn render_shadow_stable(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>, 
                               light: &light::Light, floor_y: f32) {
        let light_pos = light.position_vec3();
        if light_pos.y > floor_y + 0.1 { // Margen para evitar divisiones por cero
            let shadow_y = floor_y + 0.0001; // Offset muy pequeño para evitar z-fighting
            
            let shadow_verts: Vec<Vector3> = self.verts.iter()
                .map(|&v| {
                    let t = (v.y - shadow_y) / (light_pos.y - shadow_y);
                    Vector3::new(
                        v.x - t * (light_pos.x - v.x),
                        shadow_y,
                        v.z - t * (light_pos.z - v.z)
                    )
                })
                .collect();

            let shadow_color = Color::new(0, 0, 0, 100); // Menos transparencia para mejor visibilidad
            for (tri1, tri2, _) in self.faces.iter() {
                let face_center = (self.verts[tri1[0]] + self.verts[tri1[1]] + self.verts[tri1[2]]) / 3.0;
                if face_center.y > floor_y + 0.05 { // Solo caras claramente sobre el piso
                    d3.draw_triangle3D(
                        shadow_verts[tri1[0]], 
                        shadow_verts[tri1[1]], 
                        shadow_verts[tri1[2]], 
                        shadow_color
                    );
                    d3.draw_triangle3D(
                        shadow_verts[tri2[0]], 
                        shadow_verts[tri2[1]], 
                        shadow_verts[tri2[2]], 
                        shadow_color
                    );
                }
            }
        }
    }
}
