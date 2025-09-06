use raylib::prelude::*;

mod camera;
mod colores;
mod light;
mod framebuffer;
mod intersecto_ray;
mod cubo;
mod piso;
mod controles;
mod ui;
mod escena;

use controles::{CameraControls, LightControls};
use escena::Escena;
use light::Light;

fn main() {
    // Inicialización de la ventana
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Cubo 3D con Iluminación Dinámica")
        .build();

    rl.set_target_fps(60);

    // Inicialización de la escena
    let escena = Escena::new();
    
    // Configuración de cámara
    let mut cam = camera::make_camera();
    cam.target = Vector3::new(0.0, escena.get_cube_center_y(), 0.0);
    
    // Configuración de luz
    let mut light = Light::new([4.0, 6.0, 2.0], [1.0, 1.0, 1.0]);
    
    // Configuración de controles
    let mut camera_controls = CameraControls::new(cam.position, cam.target);
    let mut light_controls = LightControls::new();
    
    // Framebuffer
    let fb_size = (800, 600);
    let mut fb = framebuffer::create(&mut rl, &thread, fb_size.0, fb_size.1);
    
    // Estado de visualización
    let mut show_normals = false;
    let mut show_wireframe = true; // Nuevo control para mostrar/ocultar bordes

    // Loop principal
    while !rl.window_should_close() {
        // Manejo de entrada
        if rl.is_key_pressed(KeyboardKey::KEY_N) { 
            show_normals = !show_normals; 
        }
        if rl.is_key_pressed(KeyboardKey::KEY_B) { 
            show_wireframe = !show_wireframe; 
        }
        
        // Actualizar controles
        camera_controls.update(&rl);
        light_controls.update(&rl);
        
        // Actualizar posiciones
        camera_controls.update_camera_position(&mut cam);
        light_controls.update_light_position(&mut light);

        // Renderizado
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(escena.background_color);

        // Render a framebuffer
        let mut dfb = d.begin_texture_mode(&thread, &mut fb);
        dfb.clear_background(escena.background_color);
        let mut d3 = dfb.begin_mode3D(cam);

        // Renderizar escena
        escena.render(&mut d3, &light, &cam, show_normals, show_wireframe);
        
        drop(d3);
        drop(dfb);
        
        // Mostrar framebuffer en pantalla
        framebuffer::blit_to_screen(&mut d, &fb);
        
        // Renderizar UI
        ui::render_ui(&mut d, &camera_controls, &light_controls);
    }
}
