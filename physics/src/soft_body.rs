use crate::{
    body::{Body, LinearState},
    effector::Spring,
    id_pool::Id,
    world::World,
};

pub struct SoftBody {
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
    // pub fn add_soft_body(&mut self, points: Vec<LinearState>, springs: Vec<SoftBodySpring>) -> Id {
    //     let mut point_ids: Vec<Id> = Vec::new();
    //     let mut spring_ids: Vec<Id> = Vec::new();
    //
    //     for linear in points.clone() {
    //         // Set restitution
    //         point_ids.push(self.add_body(Body::new_particle(linear, 1.0)));
    //     }
    //
    //     for spring in springs {
    //         let bodies = spring.body_indices.map(|b| point_ids[b]);
    //         let length = spring.length.unwrap_or_else(|| {
    //             let (a_position, b_position) = (
    //                 points[spring.body_indices[0]].position,
    //                 points[spring.body_indices[1]].position,
    //             );
    //             a_position.metric_distance(&b_position) * 0.9
    //         });
    //
    //         let spring = Spring::new(bodies, length, spring.elasticity);
    //         spring_ids.push(self.add_effector(Box::new(spring)));
    //     }
    //
    //     let points_group = self.add_body_group(point_ids);
    //     let springs_group = self.add_body_group(spring_ids);
    //     self.add_body_group(vec![points_group, springs_group])
    // }
    //
    // pub fn remove_soft_body(&mut self, id: Id) -> Option<Vec<Id>> {
    //     self.remove_body_group(id)
    // }
    //
    // pub fn get_soft_body_points(&self, id: Id) -> Option<&Vec<Id>> {
    //     self.get_body_group(self.get_body_group(id)?[0])
    // }
    //
    // pub fn get_soft_body_springs(&self, id: Id) -> Option<&Vec<Id>> {
    //     self.get_body_group(self.get_body_group(id)?[1])
    // }
}
