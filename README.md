# RayCube - Renderizador 3D con Raylib

Un proyecto en Rust que renderiza un cubo 3D con textura, iluminación y sombras usando Raylib.

## Qué hace

- Cubo 3D con textura PNG
- Iluminación realista con sombras
- Controles para mover la cámara y luz
- Interfaz para ver información
- Modo wireframe opcional

## Cómo ejecutar

```bash
git clone https://github.com/G1LB3T0/RayCube.git
cd RayCube
cargo run
```

## Controles

### Cámara
- **W, A, S, D** o **Flechas** - Rotar cámara
- **Q / E** - Zoom
- **Mouse** - Click izquierdo y arrastra para rotar
- **Rueda del Mouse** - Zoom

### Luz
- **L** - Rotar luz automáticamente
- **+ / -** - Cambiar intensidad de luz

### UI
- **U** - Mostrar/ocultar menú
- **ESC** - Salir

## Archivos principales

```
src/
├── main.rs           # Archivo principal
├── cubo.rs          # El cubo 3D
├── escena.rs        # Manejo de la escena
├── controles.rs     # Controles del teclado/mouse
├── ui.rs            # Interfaz
└── light.rs         # Sistema de luz

assets/
└── texture1.png     # Textura del cubo
```

## Personalizar

### Cambiar textura
Reemplaza `assets/texture1.png` con tu imagen (PNG, JPG, BMP)

### Ajustar iluminación
Modifica valores en `src/colores.rs`:
```rust
pub const AMBIENT_LIGHT: f32 = 0.001;     // Luz base
pub const DIFFUSE_STRENGTH: f32 = 2.0;    // Intensidad principal
pub const SPECULAR_STRENGTH: f32 = 0.6;   // Brillos
```

## Si algo no funciona

- **No compila**: `cargo clean` y luego `cargo build`
- **No se ve textura**: Verifica que `assets/texture1.png` exista
- **Va lento**: Reduce tamaño de ventana o desactiva wireframe
