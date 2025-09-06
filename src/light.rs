use raylib::prelude::*;

pub struct Light {
	pub pos: [f32; 3],
	pub color: [f32; 3],
}

impl Light {
	pub fn new(pos: [f32; 3], color: [f32; 3]) -> Self {
		Self { pos, color }
	}
	
	pub fn position_vec3(&self) -> Vector3 {
		Vector3::new(self.pos[0], self.pos[1], self.pos[2])
	}

	pub fn render(&self, d3: &mut RaylibMode3D<RaylibTextureMode<RaylibDrawHandle>>) {
		let light_pos = self.position_vec3();
		
		// Esfera principal de la luz
		d3.draw_sphere(light_pos, 0.15, Color::new(255, 255, 100, 255));
		d3.draw_sphere_wires(light_pos, 0.18, 8, 8, Color::WHITE);
		
		// Halo de luz
		d3.draw_sphere_wires(light_pos, 0.25, 12, 12, Color::new(255, 255, 150, 150));
	}
}

pub fn color_vec(l: &Light) -> Vector3 {
	Vector3::new(l.color[0], l.color[1], l.color[2])
}

// Ray-traced shading con sombras reales y mejor contraste
pub fn shade_with_shadows(
	normal: Vector3, 
	point: Vector3,
	light: &Light, 
	base: Color, 
	light_col: Vector3, 
	ambient: f32,
	shadow_triangles: &[(Vector3, Vector3, Vector3)]
) -> Color {
	let n = normal.normalized();
	let light_pos = light.position_vec3();
	let light_dir = (light_pos - point).normalized();
	let ndotl = n.dot(light_dir).max(0.0);
	
	// Ray tracing para sombras con mayor intensidad
	let in_shadow = crate::intersecto_ray::is_in_shadow(point, light_pos, shadow_triangles);
	let shadow_factor = if in_shadow { 
		crate::colores::SHADOW_INTENSITY 
	} else { 
		1.0 
	};
	
	// Calcular difuso con contraste muy visible
	let diffuse = ndotl * ndotl * shadow_factor; // Cuadrático para buen contraste
	
	// Aplicar iluminación más directa y visible
	let light_contribution = diffuse * 2.0; // Factor x2 para mayor visibilidad
	let final_intensity = ambient + light_contribution;
	
	Color::new(
		(base.r as f32 * final_intensity * light_col.x).clamp(0.0, 255.0) as u8,
		(base.g as f32 * final_intensity * light_col.y).clamp(0.0, 255.0) as u8,
		(base.b as f32 * final_intensity * light_col.z).clamp(0.0, 255.0) as u8,
		255,
	)
}
// Módulo para luces
