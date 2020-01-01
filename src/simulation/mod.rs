use crate::body::Body;

pub mod brute_force;

pub trait Simulation {
    fn step(&mut self, bodies: &mut Vec<&mut Body>, dt: f64);
}
