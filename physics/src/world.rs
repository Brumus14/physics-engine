use std::{any::Any, collections::HashMap};

use crate::{
    body::{AngularState, Body, LinearState, Shape},
    collision::CollisionPipeline,
    effector::Effector,
    id_map::{Id, IdMap},
    types::math::Vector,
};

pub struct World {
    bodies: IdMap<Body>,
    // body_groups: HashMap<Id, Vec<Id>>,
    effectors: IdMap<Box<dyn Effector + Send + Sync>>,
    collision_pipelines: IdMap<Box<dyn CollisionPipeline + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: IdMap::new(),
            // body_groups: HashMap::new(),
            effectors: IdMap::new(),
            collision_pipelines: IdMap::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) -> Id {
        self.bodies.add(body)
    }

    pub fn remove_body(&mut self, id: Id) {
        self.bodies.remove(id);
    }

    // pub fn add_body_group(&mut self, ids: Vec<Id>) -> Id {
    //     let id = self.body_id_pool.next();
    //     self.body_groups.insert(id, ids);
    //     id
    //     self.body_groups.add(ids)
    // }
    //
    // pub fn remove_body_group(&mut self, id: Id) -> Option<Vec<Id>> {
    //     self.body_groups.remove(&id)
    // }
    //
    // pub fn get_body_group(&self, id: Id) -> Option<&Vec<Id>> {
    //     self.body_groups.get(&id)
    // }

    pub fn add_effector(&mut self, effector: Box<dyn Effector + Send + Sync>) -> Id {
        self.effectors.add(effector)
    }

    pub fn remove_effector(&mut self, id: Id) {
        self.effectors.remove(id);
    }

    pub fn get_effector(&mut self, id: Id) -> Option<&Box<dyn Effector + Send + Sync>> {
        self.effectors.get(id)
    }

    pub fn add_collision_pipeline(
        &mut self,
        collision_pipeline: Box<dyn CollisionPipeline + Send + Sync>,
    ) -> Id {
        // Init?
        self.collision_pipelines.add(collision_pipeline)
    }

    pub fn remove_collision_pipeline(&mut self, id: Id) {
        self.collision_pipelines.remove(id);
    }

    pub fn apply_effectors(&mut self) {
        for effector in self.effectors.values_mut() {
            effector.apply(&mut self.bodies);
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        // Add integrators
        for body in self.bodies.values_mut() {
            let linear = &mut body.linear;

            linear.velocity += (linear.force / linear.mass) * delta_time;
            linear.position += linear.velocity * delta_time;
            linear.force = Vector::zeros();

            let angular = &mut body.angular;

            // Is this dodgy
            if angular.inertia == 0.0 {
                continue;
            };

            angular.velocity += (angular.torque / angular.inertia) * delta_time;
            angular.rotation += angular.velocity * delta_time;
            angular.torque = 0.0;
        }
    }

    pub fn handle_collisions(&mut self) {
        for pipeline in self.collision_pipelines.values_mut() {
            // pipeline.handle(
            //     &mut self.linear_states,
            //     &self.restitutions,
            //     &mut self.angular_states,
            //     &self.shapes,
            // );
        }
    }

    pub fn get_body(&self, id: Id) -> Option<&Body> {
        self.bodies.get(id)
    }

    pub fn get_body_mut(&mut self, id: Id) -> Option<&mut Body> {
        self.bodies.get_mut(id)
    }
}
