use raylib::prelude::*;

pub type Framebuffer = RenderTexture2D;

pub fn create(rl: &mut RaylibHandle, thread: &RaylibThread, width: i32, height: i32) -> Framebuffer {
	rl.load_render_texture(thread, width as u32, height as u32)
		.expect("No se pudo crear el framebuffer")
}

// Dibuja el contenido del framebuffer a pantalla corrigiendo el flip vertical del render texture
pub fn blit_to_screen(d: &mut RaylibDrawHandle, fb: &Framebuffer) {
	let tex = fb.texture();
	let src = Rectangle::new(0.0, 0.0, tex.width() as f32, -(tex.height() as f32));
	let pos = Vector2::new(0.0, 0.0);
	d.draw_texture_rec(tex, src, pos, Color::WHITE);
}

// Módulo para framebuffer
