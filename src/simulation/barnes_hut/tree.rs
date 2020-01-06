use crate::{body::Body, simulation::barnes_hut::quad::Quad};

#[derive(Debug)]
pub struct BarnesHutTree {
    quad: Quad,
    body: Option<Body>,
    northwest: Option<Box<BarnesHutTree>>,
    northeast: Option<Box<BarnesHutTree>>,
    southeast: Option<Box<BarnesHutTree>>,
    southwest: Option<Box<BarnesHutTree>>,
}

impl BarnesHutTree {
    pub fn new(quad: Quad) -> BarnesHutTree {
        BarnesHutTree {
            quad,
            body: None,
            northwest: None,
            northeast: None,
            southeast: None,
            southwest: None,
        }
    }

    pub fn insert(&mut self, body: &Body) {
        if self.body.is_none() {
            self.body = Some(body.clone());
            return;
        }

        if !self.is_external() {
            self.body = Some(self.body.as_ref().unwrap().add(body));
            self.insert_proper_quad(&body);
            return;
        }

        self.insert_proper_quad(&self.body.clone().unwrap());
        self.insert(&body);
    }

    pub fn update_force(&self, body: &mut Body) {
        if self.is_external() {
            if let Some(current_body) = &self.body {
                body.add_force(&current_body);
            }

            return;
        }

        if self.body.is_none() {
            return;
        }

        let current_body = self.body.as_ref().unwrap();
        if (self.quad.length() / current_body.pos().dist(body.pos())) < 2.0 {
            body.add_force(current_body);
            return;
        }

        if let Some(northwest) = &self.northwest {
            northwest.update_force(body);
        }
        if let Some(southwest) = &self.southwest {
            southwest.update_force(body);
        }
        if let Some(southeast) = &self.southeast {
            southeast.update_force(body);
        }
        if let Some(northeast) = &self.northeast {
            northeast.update_force(body);
        }
    }

    pub fn is_external(&self) -> bool {
        self.northwest.is_none()
            && self.northeast.is_none()
            && self.southeast.is_none()
            && self.southwest.is_none()
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }

    pub fn northwest(&self) -> Option<&BarnesHutTree> {
        self.northwest.as_ref().map(|t| t.as_ref())
    }

    pub fn northeast(&self) -> Option<&BarnesHutTree> {
        self.northeast.as_ref().map(|t| t.as_ref())
    }

    pub fn southeast(&self) -> Option<&BarnesHutTree> {
        self.southeast.as_ref().map(|t| t.as_ref())
    }

    pub fn southwest(&self) -> Option<&BarnesHutTree> {
        self.southwest.as_ref().map(|t| t.as_ref())
    }

    fn insert_proper_quad(&mut self, body: &Body) {
        let quad_northwest = self.quad.northwest();
        if quad_northwest.contains(body.pos()) {
            self.northwest
                .get_or_insert(Box::new(BarnesHutTree::new(quad_northwest)))
                .insert(body);
            return;
        }

        let quad_northeast = self.quad.northeast();
        if quad_northeast.contains(body.pos()) {
            self.northeast
                .get_or_insert(Box::new(BarnesHutTree::new(quad_northeast)))
                .insert(body);
            return;
        }

        let quad_southwest = self.quad.southwest();
        if quad_southwest.contains(body.pos()) {
            self.southwest
                .get_or_insert(Box::new(BarnesHutTree::new(quad_southwest)))
                .insert(body);
            return;
        }

        let quad_southeast = self.quad.southeast();
        self.southeast
            .get_or_insert(Box::new(BarnesHutTree::new(quad_southeast)))
            .insert(body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn inserts_new_body_to_tree() {
        let mut tree = BarnesHutTree::new(Quad::new(Vec2::zero(), 10.0));
        let first_body = Body::new(Vec2::new(2.0, 2.0), Vec2::zero(), 5.0);
        let second_body = Body::new(Vec2::new(-2.0, -2.0), Vec2::zero(), 10.0);
        let third_body = Body::new(Vec2::new(2.0, -2.0), Vec2::zero(), 8.0);
        let fourth_body = Body::new(Vec2::new(-2.0, 2.0), Vec2::zero(), 12.0);
        let fifth_body = Body::new(Vec2::new(-2.5, 2.5), Vec2::zero(), 20.0);

        tree.insert(&first_body);
        assert_eq!(tree.is_external(), true);
        assert_eq!(tree.body(), Some(&first_body));

        tree.insert(&second_body);
        assert_eq!(tree.is_external(), false);
        assert_eq!(tree.body(), Some(&first_body.add(&second_body)));
        assert_eq!(tree.northeast().unwrap().is_external(), true);
        assert_eq!(tree.northeast().unwrap().body(), Some(&first_body));
        assert_eq!(tree.southwest().unwrap().is_external(), true);
        assert_eq!(tree.southwest().unwrap().body(), Some(&second_body));

        tree.insert(&third_body);
        assert_eq!(tree.is_external(), false);
        assert_eq!(
            tree.body(),
            Some(&first_body.add(&second_body).add(&third_body))
        );
        assert_eq!(tree.northeast().unwrap().is_external(), true);
        assert_eq!(tree.northeast().unwrap().body(), Some(&first_body));
        assert_eq!(tree.southwest().unwrap().is_external(), true);
        assert_eq!(tree.southwest().unwrap().body(), Some(&second_body));
        assert_eq!(tree.southeast().unwrap().is_external(), true);
        assert_eq!(tree.southeast().unwrap().body(), Some(&third_body));

        tree.insert(&fourth_body);
        assert_eq!(tree.is_external(), false);
        assert_eq!(
            tree.body(),
            Some(
                &first_body
                    .add(&second_body)
                    .add(&third_body)
                    .add(&fourth_body)
            )
        );
        assert_eq!(tree.northeast().unwrap().is_external(), true);
        assert_eq!(tree.northeast().unwrap().body(), Some(&first_body));
        assert_eq!(tree.southwest().unwrap().is_external(), true);
        assert_eq!(tree.southwest().unwrap().body(), Some(&second_body));
        assert_eq!(tree.southeast().unwrap().is_external(), true);
        assert_eq!(tree.southeast().unwrap().body(), Some(&third_body));
        assert_eq!(tree.northwest().unwrap().is_external(), true);
        assert_eq!(tree.northwest().unwrap().body(), Some(&fourth_body));

        tree.insert(&fifth_body);
        assert_eq!(tree.is_external(), false);
        assert_eq!(
            tree.body(),
            Some(
                &first_body
                    .add(&second_body)
                    .add(&third_body)
                    .add(&fourth_body)
                    .add(&fifth_body)
            )
        );
        assert_eq!(tree.northeast().unwrap().is_external(), true);
        assert_eq!(tree.northeast().unwrap().body(), Some(&first_body));
        assert_eq!(tree.southwest().unwrap().is_external(), true);
        assert_eq!(tree.southwest().unwrap().body(), Some(&second_body));
        assert_eq!(tree.southeast().unwrap().is_external(), true);
        assert_eq!(tree.southeast().unwrap().body(), Some(&third_body));
        assert_eq!(tree.northwest().unwrap().is_external(), false);
        assert_eq!(
            tree.northwest().unwrap().body(),
            Some(&fourth_body.add(&fifth_body))
        );
        assert_eq!(tree.northwest().unwrap().is_external(), false);
        assert_eq!(
            tree.northwest().unwrap().body(),
            Some(&fourth_body.add(&fifth_body))
        );
        assert_eq!(
            tree.northwest().unwrap().southeast().unwrap().is_external(),
            true
        );
        assert_eq!(
            tree.northwest().unwrap().southeast().unwrap().body(),
            Some(&fourth_body)
        );
        assert_eq!(
            tree.northwest().unwrap().northwest().unwrap().is_external(),
            true
        );
        assert_eq!(
            tree.northwest().unwrap().northwest().unwrap().body(),
            Some(&fifth_body)
        );
    }

    #[test]
    fn updates_force_of_a_body() {
        for (tree, expected_bodies_to_add_force) in vec![
            (BarnesHutTree::new(Quad::new(Vec2::zero(), 10.0)), vec![]),
            (
                {
                    let mut tree = BarnesHutTree::new(Quad::new(Vec2::zero(), 10.0));
                    tree.insert(&Body::new(Vec2::zero(), Vec2::zero(), 10.0));
                    tree
                },
                vec![&Body::new(Vec2::zero(), Vec2::zero(), 10.0)],
            ),
            (
                {
                    let mut tree = BarnesHutTree::new(Quad::new(Vec2::new(100.0, 100.0), 30.0));
                    tree.insert(&Body::new(Vec2::new(102.0, 101.0), Vec2::zero(), 10.0));
                    tree.insert(&Body::new(Vec2::new(99.0, 98.0), Vec2::zero(), 8.0));
                    tree
                },
                vec![
                    &Body::new(Vec2::new(102.0, 101.0), Vec2::zero(), 10.0).add(&Body::new(
                        Vec2::new(99.0, 98.0),
                        Vec2::zero(),
                        8.0,
                    )),
                ],
            ),
            (
                {
                    let mut tree = BarnesHutTree::new(Quad::new(Vec2::new(0.0, 0.0), 30.0));
                    tree.insert(&Body::new(Vec2::new(10.0, 8.0), Vec2::zero(), 10.0));
                    tree.insert(&Body::new(Vec2::new(5.0, 3.0), Vec2::zero(), 8.0));
                    tree
                },
                vec![
                    &Body::new(Vec2::new(5.0, 3.0), Vec2::zero(), 8.0),
                    &Body::new(Vec2::new(10.0, 8.0), Vec2::zero(), 10.0),
                ],
            ),
        ] {
            let mut body = Body::new(Vec2::new(10.0, 5.0), Vec2::unit(), 5.0);

            tree.update_force(&mut body);

            let mut twin_body = Body::new(Vec2::new(10.0, 5.0), Vec2::unit(), 5.0);
            for other_body in expected_bodies_to_add_force {
                twin_body.add_force(other_body);
            }

            assert_eq!(body.force(), twin_body.force());
        }
    }
}
