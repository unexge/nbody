use crate::vec2::Vec2;

#[derive(PartialEq, Debug, Clone)]
pub struct Quad {
    center: Vec2,
    length: f64,
}

impl Quad {
    pub fn new(center: Vec2, length: f64) -> Quad {
        Quad { center, length }
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        let half_length = self.length / 2.0;

        point.x() <= self.center.x() + half_length
            && point.x() >= self.center.x() - half_length
            && point.y() <= self.center.y() + half_length
            && point.y() >= self.center.y() - half_length
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn northwest(&self) -> Quad {
        let half_length = self.length / 2.0;
        let quarter_length = self.length / 4.0;

        Quad::new(
            self.center.clone() + Vec2::new(-quarter_length, quarter_length),
            half_length,
        )
    }

    pub fn northeast(&self) -> Quad {
        let half_length = self.length / 2.0;
        let quarter_length = self.length / 4.0;

        Quad::new(
            self.center.clone() + Vec2::new(quarter_length, quarter_length),
            half_length,
        )
    }

    pub fn southwest(&self) -> Quad {
        let half_length = self.length / 2.0;
        let quarter_length = self.length / 4.0;

        Quad::new(
            self.center.clone() - Vec2::new(quarter_length, quarter_length),
            half_length,
        )
    }

    pub fn southeast(&self) -> Quad {
        let half_length = self.length / 2.0;
        let quarter_length = self.length / 4.0;

        Quad::new(
            self.center.clone() + Vec2::new(quarter_length, -quarter_length),
            half_length,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_whether_contains_a_point() {
        let node = Quad::new(Vec2::zero(), 40.0);

        assert!(node.contains(&Vec2::new(15.0, 15.0)));
        assert!(node.contains(&Vec2::new(-15.0, 15.0)));
        assert!(node.contains(&Vec2::new(-15.0, -15.0)));
        assert!(node.contains(&Vec2::new(20.0, -20.0)));

        assert!(!node.contains(&Vec2::new(25.0, 15.0)));
        assert!(!node.contains(&Vec2::new(25.0, 25.0)));
        assert!(!node.contains(&Vec2::new(-25.0, 15.0)));
        assert!(!node.contains(&Vec2::new(-25.0, -15.0)));
    }

    #[test]
    fn returns_subdivisions() {
        let node = Quad::new(Vec2::zero(), 40.0);

        assert_eq!(node.northwest(), Quad::new(Vec2::new(-10.0, 10.0), 20.0));
        assert_eq!(node.northeast(), Quad::new(Vec2::new(10.0, 10.0), 20.0));
        assert_eq!(node.southwest(), Quad::new(Vec2::new(-10.0, -10.0), 20.0));
        assert_eq!(node.southeast(), Quad::new(Vec2::new(10.0, -10.0), 20.0));
    }
}
