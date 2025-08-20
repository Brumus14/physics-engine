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
        self.resolver
            .resolve(&self.bodies, collisions, linear_states, shapes);
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

pub struct DefaultNarrowPhase {}

impl DefaultNarrowPhase {
    pub fn new() -> Self {
        Self {}
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
                collisions.push(collision);
            }
        }

        println!("{:?}", collisions);

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

pub struct DefaultCollisionResolver {}

impl DefaultCollisionResolver {
    pub fn new() -> Self {
        Self {}
    }
}

impl CollisionResolution for DefaultCollisionResolver {
    fn resolve(
        &mut self,
        bodies: &Vec<Id>,
        collisions: Vec<CollisionData>,
        linear_states: &mut HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) {
        for collision in collisions {
            linear_states
                .get_mut(&collision.bodies[0])
                .unwrap()
                .position += collision.normal * collision.depth / 2.0;
            linear_states
                .get_mut(&collision.bodies[1])
                .unwrap()
                .position -= collision.normal * collision.depth / 2.0;

            linear_states
                .get_mut(&collision.bodies[0])
                .unwrap()
                .velocity *= 0.99;
            linear_states
                .get_mut(&collision.bodies[1])
                .unwrap()
                .velocity *= 0.99;
        }
    }
}
