use std::collections::HashMap;

use crate::{body::Body, force_generator::ForceGenerator, id_pool::IdPool};

pub type Id = usize;

pub struct World {
    bodies: HashMap<Id, Body>,
    // Is there a cleaner way?
    force_generators: HashMap<Id, Box<dyn ForceGenerator + Send + Sync>>,
    body_id_pool: IdPool,
    force_generator_id_pool: IdPool,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: HashMap::new(),
            force_generators: HashMap::new(),
            body_id_pool: IdPool::new(),
            force_generator_id_pool: IdPool::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) -> Id {
        let id = self.body_id_pool.next();
        self.bodies.insert(id, body);
        id
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

    pub fn step(&mut self, delta_time: f64) {
        self.bodies.values_mut().for_each(|o| o.step(delta_time));
    }

    pub fn get_body(&self, id: Id) -> Option<&Body> {
        self.bodies.get(&id)
    }

    pub fn get_body_mut(&mut self, id: Id) -> Option<&mut Body> {
        self.bodies.get_mut(&id)
    }
}
