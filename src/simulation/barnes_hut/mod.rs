pub mod quad;
pub mod tree;

use crate::{body::Body, simulation::Simulation};
use quad::Quad;
use tree::BarnesHutTree;

pub struct BarnesHut {
    quad: Quad,
}

impl BarnesHut {
    pub fn new(quad: Quad) -> BarnesHut {
        BarnesHut { quad }
    }
}

impl Simulation for BarnesHut {
    fn step(&mut self, bodies: &mut Vec<&mut Body>, dt: f64) {
        let mut tree = BarnesHutTree::new(self.quad.clone());

        for body in bodies.iter() {
            if self.quad.contains(body.pos()) {
                tree.insert(body);
            }
        }

        for body in bodies {
            body.reset_force();
            if self.quad.contains(body.pos()) {
                tree.update_force(body);
                body.update(dt);
            }
        }
    }
}
