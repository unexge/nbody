use crate::body::Body;

pub mod barnes_hut;
pub mod brute_force;

pub trait Simulation {
    fn step(&mut self, bodies: &mut Vec<&mut Body>, dt: f64);
}
