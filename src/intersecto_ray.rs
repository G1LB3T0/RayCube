use raylib::prelude::*;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction: direction.normalized() }
    }
}

// Test de intersección rayo-triángulo (Möller-Trumbore)
pub fn ray_triangle_intersect(ray: &Ray, v0: Vector3, v1: Vector3, v2: Vector3) -> Option<f32> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);
    
    if a > -f32::EPSILON && a < f32::EPSILON {
        return None; // Rayo paralelo al triángulo
    }
    
    let f = 1.0 / a;
    let s = ray.origin - v0;
    let u = f * s.dot(h);
    
    if u < 0.0 || u > 1.0 {
        return None;
    }
    
    let q = s.cross(edge1);
    let v = f * ray.direction.dot(q);
    
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    
    let t = f * edge2.dot(q);
    
    if t > f32::EPSILON {
        Some(t)
    } else {
        None
    }
}

// Test si un punto está en sombra desde una luz
pub fn is_in_shadow(point: Vector3, light_pos: Vector3, triangles: &[(Vector3, Vector3, Vector3)]) -> bool {
    let light_dir = (light_pos - point).normalized();
    let light_dist = (light_pos - point).length();
    let shadow_ray = Ray::new(point + light_dir * 0.001, light_dir); // pequeño offset para evitar self-intersection
    
    for &(v0, v1, v2) in triangles {
        if let Some(t) = ray_triangle_intersect(&shadow_ray, v0, v1, v2) {
            if t < light_dist - 0.001 {
                return true; // Hay un objeto entre el punto y la luz
            }
        }
    }
    false
}
