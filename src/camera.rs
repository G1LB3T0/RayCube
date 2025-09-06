use raylib::prelude::*;

pub fn make_camera() -> Camera3D {
	Camera3D::perspective(
		Vector3::new(5.0, 2.0, 5.0), // Posición más alta para ver el cubo en el suelo
		Vector3::new(0.0, -0.5, 0.0), // Mirar un poco hacia abajo
		Vector3::new(0.0, 1.0, 0.0),
		45.0,
	)
}

// Módulo para la cámara
