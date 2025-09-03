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

impl World {
    pub fn add_soft_body(&mut self, points: Vec<LinearState>, springs: Vec<Spring>) -> Id {
        let mut point_ids: Vec<Id> = Vec::new();
        let mut spring_ids: Vec<Id> = Vec::new();

        for linear in points {
            point_ids.push(self.add_body(Body::Particle { linear }));
        }

        for mut spring in springs {
            spring.bodies = spring.bodies.map(|b| point_ids[b]);
            spring_ids.push(self.add_effector(Box::new(spring)));
        }

        let points_group = self.add_body_group(point_ids);
        let springs_group = self.add_body_group(spring_ids);
        self.add_body_group(vec![points_group, springs_group])
    }

    pub fn remove_soft_body(&mut self, id: Id) -> Option<Vec<Id>> {
        self.remove_body_group(id)
    }

    pub fn get_soft_body_points(&self, id: Id) -> Option<&Vec<Id>> {
        self.get_body_group(self.get_body_group(id)?[0])
    }

    pub fn get_soft_body_springs(&self, id: Id) -> Option<&Vec<Id>> {
        self.get_body_group(self.get_body_group(id)?[1])
    }
}
