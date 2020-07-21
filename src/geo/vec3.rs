use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut p = self;
        p += rhs;
        p
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut p = self;
        p -= rhs;
        p
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn test_ops() {
        let p1 = Vec3::new(1.0, 2.0, 3.0);
        let p2 = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(p1 + p2, Vec3::new(0.0, 0.0, 0.0));
        let mut p3 = p1;
        p3 += p2;
        assert_eq!(p3, Vec3::new(0.0, 0.0, 0.0));

        assert_eq!(p1 - p2, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(p2 - p1, Vec3::new(-2.0, -4.0, -6.0));
        let mut p4 = p1;
        p4 -= p2;
        assert_eq!(p4, Vec3::new(2.0, 4.0, 6.0));
    }
}
