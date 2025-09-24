use crate::{
    body::{Body, LinearState},
    effector::Spring,
    id_map::Id,
    world::World,
};

pub struct SoftBodyId {
    pub points: Vec<Id>,
    pub springs: Vec<Id>,
}

#[derive(Clone)]
pub struct SoftBodySpring {
    pub body_indices: [usize; 2],
    pub length: Option<f64>,
    pub elasticity: f64,
}

impl SoftBodySpring {
    pub fn new(body_indices: [usize; 2], length: f64, elasticity: f64) -> Self {
        Self {
            body_indices,
            length: Some(length),
            elasticity,
        }
    }

    pub fn new_auto_length(body_indices: [usize; 2], elasticity: f64) -> Self {
        Self {
            body_indices,
            length: None,
            elasticity,
        }
    }
}

impl World {
    pub fn add_soft_body(
        &mut self,
        points: Vec<LinearState>,
        springs: Vec<SoftBodySpring>,
    ) -> SoftBodyId {
        let point_ids: Vec<Id> = points
            .clone()
            .into_iter()
            .map(|l| self.add_body(Body::new_particle(l, 1.0)))
            .collect();

        let spring_ids: Vec<Id> = springs
            .into_iter()
            .map(|s| {
                self.add_effector(Box::new(Spring::new(
                    s.body_indices.map(|b| point_ids[b]),
                    match s.length {
                        Some(l) => l,
                        None => {
                            let (a_position, b_position) = (
                                points[s.body_indices[0]].position,
                                points[s.body_indices[1]].position,
                            );
                            a_position.metric_distance(&b_position)
                        }
                    },
                    s.elasticity,
                )))
            })
            .collect();

        SoftBodyId {
            points: point_ids,
            springs: spring_ids,
        }
    }

    pub fn remove_soft_body(&mut self, id: SoftBodyId) {
        let SoftBodyId { points, springs } = id;

        points.into_iter().for_each(|i| self.remove_body(i));
        springs.into_iter().for_each(|i| self.remove_effector(i));
    }
}
