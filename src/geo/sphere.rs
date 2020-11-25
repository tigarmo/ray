use super::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn ray_intersect(&self, orig: Vec3, dir: Vec3) -> Option<f64> {
        let center = self.center;
        let radius = self.radius;

        let L = center - orig;
        let tca = L.dot(dir);
        let d2 = L.dot(L) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (radius * radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            if t1 < 0.0 {
                None
            } else {
                Some(t1)
            }
        } else {
            Some(t0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_intersect() {
        let origin = Vec3::origin();
        let sphere = Sphere::new(origin, 1.0);
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
