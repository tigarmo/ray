use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn origin() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        let length = self.length();
        if length == 0.0 {
            Vec3::origin()
        } else {
            self / length
        }
    }

    pub fn clamped(self) -> Vec3 {
        Vec3::new(
            self.x.clamp(0.0, 1.0),
            self.y.clamp(0.0, 1.0),
            self.z.clamp(0.0, 1.0),
        )
    }

    /// Compute the dot product between this vector and `other`
    pub fn dot(self, other: Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn reflect(self, other: Vec3) -> Vec3 {
        self - other * 2.0 * (self.dot(other))
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

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Vec3 {
        let mut p = self;
        p /= rhs;
        p
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
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
        p3 += p2;
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

    #[test]
    fn test_div() {
        let (p1, p2) = setup();
        assert_eq!(p1 / 2.0, (0.5, 1.0, 1.5));
        assert_eq!(p2 / 2.0, (-0.5, -1.0, -1.5));

        let mut p3 = p1 * 10.0;
        p3 /= 2.0;
        assert_eq!(p3, (5.0, 10.0, 15.0));
    }

    #[test]
    fn test_clamp() {
        let (p1, p2) = setup();
        assert_eq!(p1.clamped(), (1.0, 1.0, 1.0));
        assert_eq!(p2.clamped(), (0.0, 0.0, 0.0));

        let p3 = Vec3::new(-1.0, 2.0, 0.5);
        assert_eq!(p3.clamped(), (0.0, 1.0, 0.5));
    }

    #[test]
    fn test_dot() {
        let (p1, p2) = setup();
        assert_eq!(p1.dot(p1), 14.0);
        assert_eq!(p2.dot(p2), 14.0);
        assert_eq!(p1.dot(p2), -14.0);
    }

    #[test]
    fn test_length() {
        let (p1, p2) = setup();
        assert_eq!(p1.length(), 14.0_f64.sqrt());
        assert_eq!(p2.length(), 14.0_f64.sqrt());

        assert_eq!(Vec3::origin().length(), 0.0);
    }

    #[test]
    fn test_normalized() {
        assert_eq!(Vec3::origin().normalized(), (0.0, 0.0, 0.0));
        assert_eq!(Vec3::new(2.0, 0.0, 0.0).normalized(), (1.0, 0.0, 0.0));
        assert_eq!(Vec3::new(-2.0, 0.0, 0.0).normalized(), (-1.0, 0.0, 0.0));
    }
}
