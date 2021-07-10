use crate::geo::vec3::Vec3;

/// A material world
#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse: Vec3,
    pub albedo: Vec3,
    pub specular_exponent: f64,
}

impl Material {
    pub const IVORY: Material = Material {
        albedo: Vec3 {
            x: 0.6,
            y: 0.3,
            z: 0.0,
        },
        diffuse: Vec3 {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
        specular_exponent: 50.0,
    };

    pub const RED_RUBBER: Material = Material {
        albedo: Vec3 {
            x: 0.9,
            y: 0.1,
            z: 0.0,
        },
        diffuse: Vec3 {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
        specular_exponent: 10.0,
    };

    pub const INDIGO: Material = Material {
        albedo: Vec3 {
            x: 0.9,
            y: 0.1,
            z: 0.0,
        },
        diffuse: Vec3 {
            x: 75.0 / 255.0,
            y: 0.0,
            z: 130.0 / 255.0,
        },
        specular_exponent: 10.0,
    };
}
