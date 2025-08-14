use std::collections::{HashMap, VecDeque};

use crate::{
    force_generator::{ConstantAcceleration, ForceGenerator, Gravity},
    object::Object,
    types::math::*,
};

pub type ObjectId = usize;

pub struct World {
    objects: HashMap<ObjectId, Object>,
    // Is there a cleaner way?
    force_generators: Vec<Box<dyn ForceGenerator + Send + Sync>>,
    next_id: ObjectId,
    free_ids: VecDeque<ObjectId>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            // force_generators: Vec::new(),
            force_generators: Vec::new(),
            next_id: 0,
            free_ids: VecDeque::new(),
        }
    }

    fn get_next_id(&mut self) -> ObjectId {
        if let Some(id) = self.free_ids.pop_front() {
            id
        } else {
            self.next_id += 1;
            self.next_id - 1
        }
    }

    pub fn thing(&mut self) {
        self.force_generators
            .push(Box::new(ConstantAcceleration::new(
                self.objects.keys().map(|i| *i).collect(),
                Vector::new(0.0, -10.0),
            )));

        self.force_generators.push(Box::new(Gravity::new(
            self.objects.keys().map(|i| *i).collect(),
            1.0,
        )));
    }

    pub fn add(&mut self, object: Object) -> ObjectId {
        let id = self.get_next_id();
        self.objects.insert(id, object);
        self.objects.len() - 1
    }

    pub fn remove(&mut self, id: ObjectId) {
        self.objects.remove(&id);
        self.free_ids.push_back(id);
    }

    pub fn apply_forces(&mut self) {
        for generator in self.force_generators.iter_mut() {
            generator.apply(&mut self.objects);
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        self.objects.values_mut().for_each(|o| o.step(delta_time));
    }

    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }
}
