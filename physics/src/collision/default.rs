use std::rc::Rc;

use crate::collision::*;

pub struct DefaultCollisionPipeline {
    bodies: Vec<Id>,
    detector: DefaultCollisionDetector,
    resolver: DefaultCollisionResolver,
}

impl DefaultCollisionPipeline {
    pub fn new(bodies: Vec<Id>) -> Self {
        Self {
            bodies,
            detector: DefaultCollisionDetector::new(),
            resolver: DefaultCollisionResolver::new(),
        }
    }
}

impl CollisionPipeline for DefaultCollisionPipeline {
    fn handle(
        &mut self,
        linear_states: &mut HashMap<Id, LinearState>,
        angular_states: &mut HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) {
        let collisions = self.detector.detect(&self.bodies, linear_states, shapes);
        self.resolver.resolve(collisions, linear_states, shapes);
    }
}

pub struct DefaultCollisionDetector {
    broad_phase: DefaultBroadPhase,
    narrow_phase: DefaultNarrowPhase,
}

impl DefaultCollisionDetector {
    pub fn new() -> Self {
        Self {
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: DefaultNarrowPhase::new(),
        }
    }
}

impl CollisionDetection for DefaultCollisionDetector {
    fn detect(
        &mut self,
        bodies: &Vec<Id>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        let body_pairs = self.broad_phase.cull(bodies);
        self.narrow_phase
            .detect(bodies, body_pairs, linear_states, shapes)
    }
}

pub struct DefaultBroadPhase {}

impl DefaultBroadPhase {
    pub fn new() -> Self {
        Self {}
    }
}

impl BroadPhase for DefaultBroadPhase {
    fn cull(&mut self, bodies: &Vec<Id>) -> Vec<[Id; 2]> {
        let mut pairs = Vec::new();

        for i in 0..bodies.len() {
            for j in (i + 1)..bodies.len() {
                pairs.push([bodies[i], bodies[j]]);
            }
        }

        pairs
    }
}

pub struct DefaultNarrowPhase {
    collisions: u64,
}

impl DefaultNarrowPhase {
    pub fn new() -> Self {
        Self { collisions: 0 }
    }
}

impl NarrowPhase for DefaultNarrowPhase {
    fn detect(
        &mut self,
        bodies: &Vec<Id>,
        body_pairs: Vec<[Id; 2]>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        let mut collisions = Vec::new();

        for pair in body_pairs {
            let (a_linear, b_linear) = (
                linear_states.get(&pair[0]).unwrap(),
                linear_states.get(&pair[1]).unwrap(),
            );
            let (a_shape, b_shape) = (shapes.get(&pair[0]).unwrap(), shapes.get(&pair[1]).unwrap());

            if let Some(collision) = match a_shape {
                Shape::Circle(a_radius) => match b_shape {
                    Shape::Circle(b_radius) => DefaultNarrowPhase::detect_circle_circle(
                        pair[0], *a_radius, a_linear, pair[1], *b_radius, b_linear,
                    ),
                    _ => None,
                },
                _ => None,
            } {
                self.collisions += 1;
                println!("{}", self.collisions);
                collisions.push(collision);
            }
        }

        // println!("{:?}", collisions);

        collisions
    }
}

impl DefaultNarrowPhase {
    fn detect_circle_circle(
        a_id: Id,
        a_radius: f64,
        a_linear: &LinearState,
        b_id: Id,
        b_radius: f64,
        b_linear: &LinearState,
    ) -> Option<CollisionData> {
        let (a_position, b_position) = (&a_linear.position, &b_linear.position);
        let distance = a_position.metric_distance(b_position);
        let depth = a_radius + b_radius - distance;
        let normal = (a_position - b_position).normalize();

        if depth > 0.0 {
            Some(CollisionData {
                bodies: [a_id, b_id],
                // No?
                point: a_position + (a_radius - depth / 2.0) * normal,
                normal,
                depth,
            })
        } else {
            None
        }
    }
}

pub struct DefaultCollisionResolver {
    correction_level: f64,
    correction_tolerance: f64,
}

impl DefaultCollisionResolver {
    pub fn new() -> Self {
        Self {
            correction_level: 0.8,
            correction_tolerance: 0.01,
        }
    }
}

impl CollisionResolution for DefaultCollisionResolver {
    fn resolve(
        &mut self,
        collisions: Vec<CollisionData>,
        linear_states: &mut HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) {
        for collision in collisions {
            let a_id = collision.bodies[0];
            let b_id = collision.bodies[1];
            let [a_linear, b_linear] = linear_states.get_disjoint_mut([&a_id, &b_id]);
            let a_linear = a_linear.unwrap();
            let b_linear = b_linear.unwrap();

            let impulse = -(1.0 + a_linear.restitution * b_linear.restitution)
                * (a_linear.velocity - b_linear.velocity).dot(&collision.normal)
                / (1.0 / a_linear.mass + 1.0 / b_linear.mass);

            a_linear.velocity += impulse / a_linear.mass * collision.normal;
            b_linear.velocity -= impulse / b_linear.mass * collision.normal;

            // Positional correction
            if collision.depth > self.correction_tolerance {
                let correction = (collision.depth * self.correction_level * collision.normal)
                    / (1.0 / a_linear.mass + 1.0 / b_linear.mass);
                a_linear.position += correction / a_linear.mass;
                b_linear.position -= correction / b_linear.mass;
            }
        }
    }
}
