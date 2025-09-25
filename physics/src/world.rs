use crate::{
    body::{AngularState, Body, LinearState, Shape},
    collision::CollisionPipeline,
    effector::Effector,
    id_map::{Id, IdMap},
    integrator::{self, Integrator},
    types::math::Vector,
};

pub struct World {
    bodies: IdMap<Body>,
    integrators: IdMap<Box<dyn Integrator + Send + Sync>>,
    effectors: IdMap<Box<dyn Effector + Send + Sync>>,
    collision_pipelines: IdMap<Box<dyn CollisionPipeline + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: IdMap::new(),
            integrators: IdMap::new(),
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

    pub fn clear_bodies(&mut self) {
        self.bodies.clear();
    }

    pub fn get_body(&self, id: Id) -> Option<&Body> {
        self.bodies.get(id)
    }

    pub fn get_body_mut(&mut self, id: Id) -> Option<&mut Body> {
        self.bodies.get_mut(id)
    }

    // Is there a way to not keep doing + Send + Sync
    pub fn add_integrator(&mut self, integrator: Box<dyn Integrator + Send + Sync>) -> Id {
        self.integrators.add(integrator)
    }

    pub fn remove_integrator(&mut self, id: Id) {
        self.integrators.remove(id);
    }

    pub fn clear_integrators(&mut self) {
        self.integrators.clear();
    }

    pub fn get_integrator(&self, id: Id) -> Option<&Box<dyn Integrator + Send + Sync>> {
        self.integrators.get(id)
    }

    pub fn get_integrator_mut(&mut self, id: Id) -> Option<&mut Box<dyn Integrator + Send + Sync>> {
        self.integrators.get_mut(id)
    }

    pub fn step(&mut self, delta_time: f64) {
        for integrator in self.integrators.values_mut() {
            integrator.step(delta_time, &mut self.bodies);
        }

        // Reset force and torque
        for body in self.bodies.values_mut() {
            body.linear.force = Vector::zeros();
            body.angular.torque = 0.0;
        }
    }

    pub fn add_effector(&mut self, effector: Box<dyn Effector + Send + Sync>) -> Id {
        self.effectors.add(effector)
    }

    pub fn remove_effector(&mut self, id: Id) {
        self.effectors.remove(id);
    }

    pub fn clear_effectors(&mut self) {
        self.effectors.clear();
    }

    pub fn get_effector(&self, id: Id) -> Option<&Box<dyn Effector + Send + Sync>> {
        self.effectors.get(id)
    }

    pub fn get_effector_mut(&mut self, id: Id) -> Option<&mut Box<dyn Effector + Send + Sync>> {
        self.effectors.get_mut(id)
    }

    pub fn apply_effectors(&mut self) {
        for effector in self.effectors.values_mut() {
            effector.apply(&mut self.bodies);
        }
    }

    pub fn add_collision_pipeline(
        &mut self,
        mut collision_pipeline: Box<dyn CollisionPipeline + Send + Sync>,
    ) -> Id {
        collision_pipeline.init(&mut self.bodies);
        self.collision_pipelines.add(collision_pipeline)
    }

    pub fn remove_collision_pipeline(&mut self, id: Id) {
        self.collision_pipelines.remove(id);
    }

    pub fn clear_collision_pipelines(&mut self) {
        self.collision_pipelines.clear();
    }

    pub fn get_collision_pipeline(
        &self,
        id: Id,
    ) -> Option<&Box<dyn CollisionPipeline + Send + Sync>> {
        self.collision_pipelines.get(id)
    }

    pub fn get_collision_pipeline_mut(
        &mut self,
        id: Id,
    ) -> Option<&mut Box<dyn CollisionPipeline + Send + Sync>> {
        self.collision_pipelines.get_mut(id)
    }

    pub fn handle_collisions(&mut self) {
        for pipeline in self.collision_pipelines.values_mut() {
            pipeline.handle(&mut self.bodies);
        }
    }

    pub fn reset(&mut self) {
        self.clear_collision_pipelines();
        self.clear_effectors();
        self.clear_integrators();
        self.clear_bodies();
    }
}
