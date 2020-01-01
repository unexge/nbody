use super::Simulation;
use crate::body::Body;
use std::cell::RefCell;

pub struct BruteForce;

impl BruteForce {
    pub fn new() -> BruteForce {
        BruteForce
    }
}

impl Simulation for BruteForce {
    fn step(&mut self, bodies: &mut Vec<&mut Body>, dt: f64) {
        {
            let bodies: Vec<RefCell<_>> = bodies.iter_mut().map(|b| RefCell::new(b)).collect();

            for i in 0..bodies.len() {
                let mut body = bodies[i].borrow_mut();
                body.reset_force();

                for j in 0..bodies.len() {
                    if j == i {
                        continue;
                    }

                    let other_body = bodies[j].borrow();
                    body.add_force(&other_body);
                }
            }
        }

        for body in bodies {
            body.update(dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn calculates_next_state() {
        let mut simulation = BruteForce::new();

        let mut body_1 = Body::new(Vec2::new(10.0, 9.0), Vec2::unit(), 10.0);
        let mut body_2 = Body::new(Vec2::new(7.0, 2.0), Vec2::unit(), 12.0);
        let mut body_3 = Body::new(Vec2::new(5.0, 7.0), Vec2::new(2.0, 1.5), 8.0);

        let mut bodies = vec![&mut body_1, &mut body_2, &mut body_3];

        simulation.step(&mut bodies, 1e11);

        assert_eq!(
            bodies.iter().map(|b| b.force()).collect::<Vec<&Vec2>>(),
            vec![
                &Vec2::new(2.2533832820136364e-10, 1.9529728777099549e-10),
                &Vec2::new(2.7659106706034156e-11, -3.3205265799121046e-10),
                &Vec2::new(-2.5299743490739776e-10, 1.3675537022021497e-10)
            ]
        );

        assert_eq!(
            bodies.iter().map(|b| b.velocity()).collect::<Vec<&Vec2>>(),
            vec![
                &Vec2::new(3.2533832820136364, 2.952972877709955),
                &Vec2::new(1.230492555883618, -1.767105483260087),
                &Vec2::new(-1.162467936342472, 3.2094421277526872)
            ]
        );

        assert_eq!(
            bodies.iter().map(|b| b.pos()).collect::<Vec<&Vec2>>(),
            vec![
                &Vec2::new(325338328211.36365, 295297287779.9955),
                &Vec2::new(123049255595.3618, -176710548324.0087),
                &Vec2::new(-116246793629.2472, 320944212782.26874)
            ]
        );
    }
}
