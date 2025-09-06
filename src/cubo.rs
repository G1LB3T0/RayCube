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
        // Cubo base extremadamente oscuro para contraste máximo
        let cube_color = Color::new(
            (colores::OBJECT_COLOR[0] * 15.0) as u8, // Extremadamente oscuro
            (colores::OBJECT_COLOR[1] * 15.0) as u8,
            (colores::OBJECT_COLOR[2] * 15.0) as u8,
            255
        );
        d3.draw_cube(Vector3::new(0.0, self.position_offset, 0.0), self.size * 2.0, self.size * 2.0, self.size * 2.0, cube_color);
    }

    pub fn render_lit_faces(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>, 
                           light: &light::Light, cam: &Camera3D, show_normals: bool) {
        // Crear lista de triángulos para shadow testing
        let mut all_triangles = Vec::new();
        for (tri1, tri2, _) in self.faces.iter() {
            all_triangles.push((self.verts[tri1[0]], self.verts[tri1[1]], self.verts[tri1[2]]));
            all_triangles.push((self.verts[tri2[0]], self.verts[tri2[1]], self.verts[tri2[2]]));
        }

        // Desactivar backface culling completamente para este renderizado
        unsafe { 
            raylib::ffi::rlDisableBackfaceCulling();
            raylib::ffi::rlDisableDepthTest(); // También desactivar depth test temporalmente
        }
        
        for (tri1, tri2, normal_hint) in self.faces.iter() {
            let normal = *normal_hint;
            
            // Primer triángulo
            let (a, b, c) = (self.verts[tri1[0]], self.verts[tri1[1]], self.verts[tri1[2]]);
            let center1 = (a + b + c) / 3.0;
            
            // Dibujar SIEMPRE, sin importar la orientación
            let color1 = calculate_realistic_lighting(normal, center1, light, cam.position);
            d3.draw_triangle3D(a, b, c, color1);

            // Segundo triángulo
            let (a2, b2, c2) = (self.verts[tri2[0]], self.verts[tri2[1]], self.verts[tri2[2]]);
            let center2 = (a2 + b2 + c2) / 3.0;
            
            // Dibujar SIEMPRE, sin importar la orientación
            let color2 = calculate_realistic_lighting(normal, center2, light, cam.position);
            d3.draw_triangle3D(a2, b2, c2, color2);

            if show_normals {
                let ctri = (a + b + c) / 3.0;
                d3.draw_line_3D(ctri, ctri + normal * 0.6, Color::YELLOW);
                let ctri2 = (a2 + b2 + c2) / 3.0;
                d3.draw_line_3D(ctri2, ctri2 + normal * 0.6, Color::YELLOW);
            }
        }
        
        // Reactivar el depth test pero NO el backface culling
        unsafe { 
            raylib::ffi::rlEnableDepthTest();
            // Mantener backface culling desactivado: raylib::ffi::rlEnableBackfaceCulling();
        }
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

// Función de iluminación extrema para contraste máximo con color rojo fijo
fn calculate_realistic_lighting(normal: Vector3, point: Vector3, light: &light::Light, camera_pos: Vector3) -> Color {
    let light_pos = light.position_vec3();
    let light_dir = (light_pos - point).normalized();
    let n = normal.normalized();
    let view_dir = (camera_pos - point).normalized();
    
    // Calcular distancia para atenuación
    let distance = (light_pos - point).length();
    let attenuation = 1.0 / (1.0 + 0.05 * distance);
    
    // Iluminación difusa con contraste extremo
    let dot_product = n.dot(light_dir);
    
    // Para evitar que las caras desaparezcan, usar el valor absoluto del dot product
    // Esto hace que ambos lados de la cara reciban algo de iluminación
    let diffuse_raw = dot_product.abs().max(0.1); // Mínimo 0.1 para visibilidad
    
    // Sin luz ambiente prácticamente, solo luz directa
    let diffuse_intensity = (diffuse_raw * diffuse_raw * attenuation * colores::DIFFUSE_STRENGTH).min(3.0);
    
    // Iluminación especular muy brillante
    let reflect_dir = reflect_vector(-light_dir, n);
    let spec_factor = view_dir.dot(reflect_dir).max(0.0);
    let spec_intensity = (spec_factor.powf(32.0) * attenuation * colores::SPECULAR_STRENGTH).min(1.0);
    
    // Luz ambiente prácticamente nula pero con mínimo para visibilidad
    let ambient = colores::AMBIENT_LIGHT.max(0.05); // Mínimo 0.05 para que siempre sea visible
    
    // Definir color rojo base fijo
    let red_base = Color::new(255, 0, 0, 255); // Rojo puro
    
    // Combinar componentes manteniendo el color rojo
    let final_intensity = ambient + diffuse_intensity;
    let final_specular = spec_intensity;
    
    // Aplicar iluminación solo al canal rojo, mantener verde y azul en 0
    let r = ((red_base.r as f32 * final_intensity * 0.5) + (255.0 * final_specular)).clamp(0.0, 255.0) as u8;
    let g = (final_specular * 50.0).clamp(0.0, 255.0) as u8; // Pequeña cantidad de verde en reflejos
    let b = (final_specular * 50.0).clamp(0.0, 255.0) as u8; // Pequeña cantidad de azul en reflejos
    
    Color::new(r, g, b, 255)
}

// Función auxiliar para calcular reflexión
fn reflect_vector(incident: Vector3, normal: Vector3) -> Vector3 {
    incident - normal * (2.0 * incident.dot(normal))
}
