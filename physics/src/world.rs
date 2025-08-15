use std::collections::HashMap;

use crate::{force_generator::ForceGenerator, id_pool::IdPool, object::Object};

pub type Id = usize;

pub struct World {
    objects: HashMap<Id, Object>,
    // Is there a cleaner way?
    force_generators: HashMap<Id, Box<dyn ForceGenerator + Send + Sync>>,
    object_id_pool: IdPool,
    force_generator_id_pool: IdPool,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            force_generators: HashMap::new(),
            object_id_pool: IdPool::new(),
            force_generator_id_pool: IdPool::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) -> Id {
        let id = self.object_id_pool.next();
        self.objects.insert(id, object);
        id
    }

    pub fn remove_object(&mut self, id: Id) {
        self.objects.remove(&id);
        self.object_id_pool.free(id);
    }

    pub fn add_force_generator(&mut self, generator: Box<dyn ForceGenerator + Send + Sync>) -> Id {
        let id = self.force_generator_id_pool.next();
        self.force_generators.insert(id, generator);
        id
    }

    pub fn remove_force_generator(&mut self, id: Id) {
        self.force_generators.remove(&id);
        self.object_id_pool.free(id);
    }

    pub fn apply_forces(&mut self) {
        for generator in self.force_generators.values_mut() {
            generator.apply(&mut self.objects);
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        self.objects.values_mut().for_each(|o| o.step(delta_time));
    }

    pub fn get_object(&self, id: Id) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_object_mut(&mut self, id: Id) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }
}
