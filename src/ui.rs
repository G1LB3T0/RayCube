use raylib::prelude::*;
use crate::controles::{CameraControls, LightControls};

pub fn render_ui(d: &mut RaylibDrawHandle, camera_controls: &CameraControls, light_controls: &LightControls) {
    // Información en pantalla
    d.draw_fps(10, 10);
    d.draw_text("Controles:", 10, 40, 20, Color::WHITE);
    d.draw_text("Flechas/WASD: Rotar cámara", 10, 65, 16, Color::LIGHTGRAY);
    d.draw_text("Q/E: Zoom in/out", 10, 85, 16, Color::LIGHTGRAY);
    d.draw_text("Mouse: Click + drag para rotar", 10, 105, 16, Color::LIGHTGRAY);
    d.draw_text("Rueda: Zoom", 10, 125, 16, Color::LIGHTGRAY);
    d.draw_text("N: Mostrar normales", 10, 145, 16, Color::LIGHTGRAY);
    d.draw_text("B: Alternar bordes", 10, 165, 16, Color::LIGHTGRAY);
    d.draw_text("L: Pausar/reanudar rotación luz", 10, 185, 16, Color::LIGHTGRAY);
    
    // Información de posición
    d.draw_text(&format!("Radius: {:.2}", camera_controls.radius), 10, 205, 16, Color::YELLOW);
    d.draw_text(&format!("Yaw: {:.2}°", camera_controls.yaw.to_degrees()), 10, 225, 16, Color::YELLOW);
    d.draw_text(&format!("Pitch: {:.2}°", camera_controls.pitch.to_degrees()), 10, 245, 16, Color::YELLOW);
    
    // Estado de la luz
    let light_status = if light_controls.auto_rotate { "Rotando" } else { "Estática" };
    d.draw_text(&format!("Luz: {}", light_status), 10, 265, 16, Color::CYAN);
}
