use crate::geo::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse: Vec3,
}

impl Material {
    pub const IVORY: Material = Material {
        diffuse: Vec3 {
            x: 0.4,
            y: 0.4,
            z: 0.3,
        },
    };

    pub const RED_RUBBER: Material = Material {
        diffuse: Vec3 {
            x: 0.3,
            y: 0.1,
            z: 0.1,
        },
    };

    pub const INDIGO: Material = Material {
        diffuse: Vec3 {
            x: 75.0 / 255.0,
            y: 0.0,
            z: 130.0 / 255.0,
        },
    };
}
