use std::collections::HashMap;

use crate::{
    body::{AngularState, Body, LinearState, Shape},
    force_generator::ForceGenerator,
    id_pool::IdPool,
};

pub type Id = usize;

pub struct World {
    body_id_pool: IdPool,
    linear_states: HashMap<Id, LinearState>,
    angular_states: HashMap<Id, AngularState>,
    shapes: HashMap<Id, Shape>,
    // Is there a cleaner way?
    force_generators: HashMap<Id, Box<dyn ForceGenerator + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            body_id_pool: IdPool::new(),
            linear_states: HashMap::new(),
            angular_states: HashMap::new(),
            shapes: HashMap::new(),
            force_generators: HashMap::new(),
        }
    }

    pub fn add_body(&mut self, body: Box<dyn Body>) -> Id {
        self.body_id_pool.next()
    }

    pub fn remove_body(&mut self, id: Id) {
        self.bodies.remove(&id);
        self.body_id_pool.free(id);
    }

    pub fn add_force_generator(&mut self, generator: Box<dyn ForceGenerator + Send + Sync>) -> Id {
        let id = self.force_generator_id_pool.next();
        self.force_generators.insert(id, generator);
        id
    }

    pub fn remove_force_generator(&mut self, id: Id) {
        self.force_generators.remove(&id);
        self.body_id_pool.free(id);
    }

    pub fn apply_forces(&mut self) {
        for generator in self.force_generators.values_mut() {
            generator.apply(&mut self.bodies);
        }
    }

    // pub fn step(&mut self, delta_time: f64) {
    //     self.bodies.values_mut().for_each(|o| o.step(delta_time));
    // }

    pub fn get_body(&self, id: Id) -> Option<&Box<dyn Body>> {
        self.bodies.get(&id)
    }

    pub fn get_body_mut(&mut self, id: Id) -> Option<&mut Box<dyn Body>> {
        self.bodies.get_mut(&id)
    }
}
