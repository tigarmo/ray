use super::vec3::Vec3;

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

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn ray_intersect(&self, orig: Vec3, dir: Vec3) -> Option<f64> {
        let center = self.center;
        let radius2 = self.radius * self.radius;

        let l = center - orig;
        let tca = l.dot(dir);
        let d2 = l.dot(l) - tca * tca;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        match (t0, t1) {
            (t0, t1) if t0 < 0.0 && t1 < 0.0 => None,
            (t0, t1) if t0 < 0.0 => Some(t1),
            (t0, _) => Some(t0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let origin = Vec3::origin();
        let sphere = Sphere::new(origin, 1.0, Material::INDIGO);
        let dir = Vec3::new(0.0, 0.0, -1.0);

        assert_eq!(sphere.ray_intersect(Vec3::new(-2.0, 0.0, 1.0), dir), None);
        assert_eq!(
            sphere.ray_intersect(Vec3::new(0.0, 0.0, 1.0), dir),
            Some(0.0)
        );
        assert_eq!(
            sphere.ray_intersect(Vec3::new(0.0, 0.0, 3.0), dir),
            Some(2.0)
        );
    }
}
