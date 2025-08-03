use crate::object::Object;

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }

    pub fn step(&mut self, delta_time: f64) {
        for object in &mut self.objects {
            object.step(delta_time);
        }
    }

    pub fn get(&self, id: usize) -> Option<&Object> {
        if id < self.objects.len() {
            Some(&self.objects[id])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Object> {
        if id < self.objects.len() {
            Some(&mut self.objects[id])
        } else {
            None
        }
    }
}
