use std::collections::HashMap;

use crate::{
    body::{AngularState, Body, LinearState, Shape},
    collision::CollisionPipeline,
    effector::Effector,
    id_pool::{Id, IdPool},
    types::math::Vector,
};

pub struct World {
    body_id_pool: IdPool,
    linear_states: HashMap<Id, LinearState>,
    restitutions: HashMap<Id, f64>,
    angular_states: HashMap<Id, AngularState>,
    shapes: HashMap<Id, Shape>,
    body_groups: HashMap<Id, Vec<Id>>,
    effector_id_pool: IdPool,
    effectors: HashMap<Id, Box<dyn Effector + Send + Sync>>,
    collision_pipeline_id_pool: IdPool,
    collision_pipelines: HashMap<Id, Box<dyn CollisionPipeline + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            body_id_pool: IdPool::new(),
            linear_states: HashMap::new(),
            restitutions: HashMap::new(),
            angular_states: HashMap::new(),
            shapes: HashMap::new(),
            body_groups: HashMap::new(),
            effector_id_pool: IdPool::new(),
            effectors: HashMap::new(),
            collision_pipeline_id_pool: IdPool::new(),
            collision_pipelines: HashMap::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) -> Id {
        match body {
            Body::Particle { linear } => {
                let id = self.body_id_pool.next();
                self.linear_states.insert(id, linear);
                id
            }
            Body::Rigid {
                linear,
                angular,
                restitution,
                shape,
            } => {
                let id = self.body_id_pool.next();
                self.linear_states.insert(id, linear);
                self.restitutions.insert(id, restitution);
                self.angular_states.insert(id, angular);
                self.shapes.insert(id, shape);
                id
            } // // Combine body and effector ids
              // // Or
              // // Make constraint trait instead of using effector
              // Body::Soft { points, springs } => {
              //     let mut point_ids = Vec::new();
              //     let mut spring_ids = Vec::new();
              //
              //     for point in points {
              //         let id = self.body_id_pool.next();
              //         self.linear_states.insert(id, point);
              //         point_ids.push(id);
              //     }
              //
              //     // for mut spring in springs {
              //     //     spring.bodies = spring.bodies.map(|i| point_ids.get(i));
              //     //     let id = self.effector_id_pool.next();
              //     //     self.effectors.insert(id, Box::new(spring));
              //     //     spring_ids.push(id);
              //     // }
              // }
        }
    }

    pub fn remove_body(&mut self, id: Id) {
        self.linear_states.remove(&id);
        self.linear_states.remove(&id);
        self.angular_states.remove(&id);
        self.shapes.remove(&id);
        self.body_id_pool.free(id);
    }

    pub fn add_body_group(&mut self, ids: Vec<Id>) -> Id {
        let id = self.body_id_pool.next();
        self.body_groups.insert(id, ids);
        id
    }

    pub fn remove_body_group(&mut self, group_id: Id) -> Option<Vec<Id>> {
        self.body_groups.remove(&group_id)
    }

    pub fn add_effector(&mut self, effector: Box<dyn Effector + Send + Sync>) -> Id {
        let id = self.effector_id_pool.next();
        self.effectors.insert(id, effector);
        id
    }

    pub fn remove_effector(&mut self, id: Id) {
        self.effectors.remove(&id);
        self.body_id_pool.free(id);
    }

    pub fn add_collision_pipeline(
        &mut self,
        collision_pipeline: Box<dyn CollisionPipeline + Send + Sync>,
    ) -> Id {
        let id = self.collision_pipeline_id_pool.next();
        self.collision_pipelines.insert(id, collision_pipeline);
        id
    }

    pub fn remove_collision_pipeline(&mut self, id: Id) {
        self.collision_pipelines.remove(&id);
        self.body_id_pool.free(id);
    }

    pub fn apply_effectors(&mut self) {
        for effector in self.effectors.values_mut() {
            effector.apply(&mut self.linear_states, &mut self.angular_states);
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        for linear in self.linear_states.values_mut() {
            linear.velocity += (linear.force / linear.mass) * delta_time;
            linear.position += linear.velocity * delta_time;
            linear.force = Vector::zeros();
        }

        for angular in self.angular_states.values_mut() {
            angular.velocity += (angular.torque / angular.inertia) * delta_time;
            angular.rotation += angular.velocity * delta_time;
            angular.torque = 0.0;
        }
    }

    pub fn handle_collisions(&mut self) {
        for pipeline in self.collision_pipelines.values_mut() {
            pipeline.handle(
                &mut self.linear_states,
                &mut self.angular_states,
                &self.shapes,
            );
        }
    }

    pub fn get_linear(&self, id: Id) -> Option<&LinearState> {
        self.linear_states.get(&id)
    }

    pub fn get_linear_mut(&mut self, id: Id) -> Option<&mut LinearState> {
        self.linear_states.get_mut(&id)
    }

    pub fn get_angular(&self, id: Id) -> Option<&AngularState> {
        self.angular_states.get(&id)
    }

    pub fn get_angular_mut(&mut self, id: Id) -> Option<&mut AngularState> {
        self.angular_states.get_mut(&id)
    }

    pub fn get_shape(&self, id: Id) -> Option<&Shape> {
        self.shapes.get(&id)
    }

    pub fn get_shape_mut(&mut self, id: Id) -> Option<&mut Shape> {
        self.shapes.get_mut(&id)
    }
}
