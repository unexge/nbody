const G: f64 = 6.67408e-11;

use crate::vec2::Vec2;

#[derive(PartialEq, Debug)]
pub struct Body {
    pub pos: Vec2,
    velocity: Vec2,
    force: Vec2,
    mass: f64,
}

impl Body {
    pub fn new(pos: Vec2, velocity: Vec2, mass: f64) -> Body {
        Body {
            pos,
            velocity,
            mass,
            force: Vec2::zero(),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.velocity = self.velocity.clone() + self.force.clone() * dt / self.mass;
        self.pos = self.pos.clone() + self.velocity.clone() * dt;
    }

    pub fn add_force(&mut self, other: &Body) {
        let diff = self.pos.clone() - other.pos.clone();
        let dist = self.pos.dist(&other.pos);

        let force = (G * self.mass * other.mass) / dist.powi(2);

        self.force = diff * force / dist;
    }

    pub fn reset_force(&mut self) {
        self.force = Vec2::zero();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_by_delta_time() {
        let mut body = Body::new(Vec2::new(10.0, 9.0), Vec2::unit(), 10.0);
        body.force = Vec2::new(10.0, 8.0);

        body.update(0.16);

        assert_eq!(body.velocity, Vec2::new(1.16, 1.1280000000000001));
        assert_eq!(body.pos, Vec2::new(10.1856, 9.18048));
    }

    #[test]
    fn adds_force_from_another_body() {
        let mut first_body = Body::new(Vec2::new(10.0, 9.0), Vec2::unit(), 10.0);
        let second_body = Body::new(Vec2::new(7.0, 2.0), Vec2::unit(), 12.0);

        first_body.add_force(&second_body);

        assert_eq!(
            first_body.force,
            Vec2::new(5.439411542609485e-11, 1.2691960266088797e-10)
        );
    }

    #[test]
    fn resets_its_force() {
        let mut body = Body::new(Vec2::unit(), Vec2::unit(), 10.0);
        body.force = Vec2::new(10.0, 8.0);

        body.reset_force();

        assert_eq!(body.force, Vec2::zero());
    }
}
