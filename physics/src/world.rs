use nalgebra::Vector2;

use crate::object::Object;

const GRAVITATIONAL_CONSTANT: f64 = 1.0;

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
        let mut accelerations = vec![Vector2::new(0.0, 0.0); self.objects.len()];

        for i in 0..(self.objects.len() - 1) {
            for j in (i + 1)..self.objects.len() {
                let (a, b) = (&self.objects[i], &self.objects[j]);

                let direction = b.position - a.position;
                // TODO: Review this
                let distance_squared = direction.norm_squared().max(0.0001);

                let force = GRAVITATIONAL_CONSTANT / distance_squared;

                accelerations[i] += direction.normalize() * force * b.mass;
                accelerations[j] -= direction.normalize() * force * a.mass;
            }
        }

        for (object, acceleration) in self.objects.iter_mut().zip(accelerations) {
            object.acceleration = acceleration;
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
