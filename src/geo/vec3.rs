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

// This one is just to simplify tests
impl PartialEq<(f64, f64, f64)> for Vec3 {
    fn eq(&self, other: &(f64, f64, f64)) -> bool {
        self.x == other.0 && self.y == other.1 && self.z == other.2
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

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Vec3 {
        let mut p = self;
        p *= rhs;
        p
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    fn setup() -> (Vec3, Vec3) {
        (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0))
    }
    #[test]
    fn test_add() {
        let (p1, p2) = setup();
        assert_eq!(p1 + p2, (0.0, 0.0, 0.0));
        assert_eq!(p2 + p1, (0.0, 0.0, 0.0));
        let mut p3 = p1;
        //p3 += p2;
        assert_eq!(p3, (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_sub() {
        let (p1, p2) = setup();
        assert_eq!(p1 - p2, (2.0, 4.0, 6.0));
        assert_eq!(p2 - p1, (-2.0, -4.0, -6.0));
        let mut p4 = p1;
        p4 -= p2;
        assert_eq!(p4, (2.0, 4.0, 6.0));
    }

    #[test]
    fn test_mul() {
        let (p1, p2) = setup();
        assert_eq!(p1 * 2.0, (2.0, 4.0, 6.0));
        let mut p3 = p2;
        p3 *= 3.0;
        assert_eq!(p3, (-3.0, -6.0, -9.0));
    }
}
