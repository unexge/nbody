use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug, Clone)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn unit() -> Vec2 {
        Vec2::new(1.0, 1.0)
    }

    pub fn dist(&self, other: &Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x() * rhs.x(), self.y() * rhs.y())
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x() * rhs, self.y() * rhs)
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        Vec2::new(self.x() / rhs, self.y() / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new_instance() {
        let vec = Vec2::new(42.0, 16.25);

        assert_eq!(vec.x(), 42.0);
        assert_eq!(vec.y(), 16.25);
    }

    #[test]
    fn creates_zero_instance() {
        assert_eq!(Vec2::zero(), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn creates_unit_instance() {
        assert_eq!(Vec2::unit(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn multiplies_with_scalar() {
        assert_eq!(Vec2::new(42.0, 16.25) * 2.0, Vec2::new(84.0, 32.5));
    }

    #[test]
    fn multiplies_with_another() {
        assert_eq!(
            Vec2::new(42.0, 16.25) * Vec2::new(3.0, 1.5),
            Vec2::new(126.0, 24.375)
        );
    }

    #[test]
    fn divides_by_scalar() {
        assert_eq!(Vec2::new(42.0, 16.25) / 2.0, Vec2::new(21.0, 8.125));
    }

    #[test]
    fn subtracts_from_another() {
        assert_eq!(
            Vec2::new(42.0, 16.25) - Vec2::new(11.21, 10.02),
            Vec2::new(30.79, 6.23)
        );
    }

    #[test]
    fn adds_with_another() {
        assert_eq!(
            Vec2::new(42.0, 16.25) + Vec2::new(11.21, 10.02),
            Vec2::new(53.21, 26.27)
        );
    }

    #[test]
    fn calculates_distance() {
        assert_eq!(
            Vec2::new(42.0, 16.25).dist(&Vec2::new(11.21, 10.2)),
            31.378760332428683
        );
    }
}
