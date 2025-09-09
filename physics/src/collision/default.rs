use std::f64;

use uom::si::inverse_velocity::minute_per_foot;

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
        restitutions: &HashMap<Id, f64>,
        angular_states: &mut HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) {
        let collisions = self
            .detector
            .detect(&self.bodies, linear_states, angular_states, shapes);
        self.resolver
            .resolve(collisions, linear_states, restitutions, shapes);
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
        angular_states: &HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        let body_pairs = self.broad_phase.cull(bodies);
        self.narrow_phase
            .detect(bodies, body_pairs, linear_states, angular_states, shapes)
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
        _bodies: &Vec<Id>,
        body_pairs: Vec<[Id; 2]>,
        linear_states: &HashMap<Id, LinearState>,
        angular_states: &HashMap<Id, AngularState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        let mut collisions = Vec::new();

        for pair in body_pairs {
            let (a_linear, b_linear) = (
                linear_states.get(&pair[0]).unwrap(),
                linear_states.get(&pair[1]).unwrap(),
            );
            let (a_angular, b_angular) = (
                angular_states.get(&pair[0]).unwrap(),
                angular_states.get(&pair[1]).unwrap(),
            );
            let (a_shape, b_shape) = (shapes.get(&pair[0]).unwrap(), shapes.get(&pair[1]).unwrap());

            if let Some(collision) = match a_shape {
                Shape::Circle(a_radius) => match b_shape {
                    Shape::Circle(b_radius) => DefaultNarrowPhase::detect_circle_circle(
                        pair[0],
                        pair[1],
                        *a_radius,
                        *b_radius,
                        &a_linear.position,
                        &b_linear.position,
                    ),
                    _ => None,
                },
                Shape::Rectangle(a_size) => match b_shape {
                    Shape::Rectangle(b_size) => DefaultNarrowPhase::detect_sat(
                        pair[0],
                        pair[1],
                        vec![
                            a_linear.position
                                + Rotation::new(-a_angular.rotation)
                                    * Vector::new(-a_size.x / 2.0, a_size.y / 2.0),
                            a_linear.position
                                + Rotation::new(-a_angular.rotation)
                                    * Vector::new(a_size.x / 2.0, a_size.y / 2.0),
                            a_linear.position
                                + Rotation::new(-a_angular.rotation)
                                    * Vector::new(a_size.x / 2.0, -a_size.y / 2.0),
                            a_linear.position
                                + Rotation::new(-a_angular.rotation)
                                    * Vector::new(-a_size.x / 2.0, -a_size.y / 2.0),
                        ],
                        vec![
                            b_linear.position
                                + Rotation::new(-b_angular.rotation)
                                    * Vector::new(-b_size.x / 2.0, b_size.y / 2.0),
                            b_linear.position
                                + Rotation::new(-b_angular.rotation)
                                    * Vector::new(b_size.x / 2.0, b_size.y / 2.0),
                            b_linear.position
                                + Rotation::new(-b_angular.rotation)
                                    * Vector::new(b_size.x / 2.0, -b_size.y / 2.0),
                            b_linear.position
                                + Rotation::new(-b_angular.rotation)
                                    * Vector::new(-b_size.x / 2.0, -b_size.y / 2.0),
                        ],
                        &a_linear.position,
                        &b_linear.position,
                        vec![Vector::new(0.0, 1.0), Vector::new(1.0, 0.0)],
                        vec![Vector::new(0.0, 1.0), Vector::new(1.0, 0.0)],
                        a_angular.rotation,
                        b_angular.rotation,
                    ),
                    _ => None,
                },
                _ => None,
            } {
                self.collisions += 1;
                // println!("{:?}", collision);
                collisions.push(collision);
            }
        }

        collisions
    }
}

impl DefaultNarrowPhase {
    fn detect_circle_circle(
        a_id: Id,
        b_id: Id,
        a_radius: f64,
        b_radius: f64,
        a_position: &Vector<f64>,
        b_position: &Vector<f64>,
    ) -> Option<CollisionData> {
        let distance = a_position.metric_distance(b_position);
        let depth = a_radius + b_radius - distance;
        let normal = (b_position - a_position).normalize();

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

    // Separating Axis Theorem (SAT)
    // Take in normals?
    // Optimise
    pub fn detect_sat(
        a_id: Id,
        b_id: Id,
        a_points: Vec<Vector<f64>>,
        b_points: Vec<Vector<f64>>,
        a_position: &Vector<f64>,
        b_position: &Vector<f64>,
        a_normals: Vec<Vector<f64>>,
        b_normals: Vec<Vector<f64>>,
        a_rotation: f64,
        b_rotation: f64,
    ) -> Option<CollisionData> {
        // Pass in axes maybe
        let mut axes: Vec<Vector<f64>> = Vec::new();

        for normal in a_normals {
            axes.push(Rotation::new(-a_rotation) * normal);
        }

        for normal in b_normals {
            axes.push(Rotation::new(-b_rotation) * -normal);
        }

        let mut min_penetration = f64::INFINITY;
        let mut min_axis: Option<&Vector<f64>> = None;

        for axis in &axes {
            let mut a_min = f64::INFINITY;
            let mut a_max = f64::NEG_INFINITY;
            let mut b_min = f64::INFINITY;
            let mut b_max = f64::NEG_INFINITY;

            for point in &a_points {
                let projection = point.dot(axis);

                if projection < a_min {
                    a_min = projection;
                }

                if projection > a_max {
                    a_max = projection;
                }
            }

            for point in &b_points {
                let projection = point.dot(axis);

                if projection < b_min {
                    b_min = projection;
                }

                if projection > b_max {
                    b_max = projection;
                }
            }

            let penetration = a_max.min(b_max) - a_min.max(b_min);

            if b_min >= a_max || a_min >= b_max {
                return None;
            } else {
                if penetration < min_penetration {
                    min_penetration = penetration;
                    min_axis = Some(axis);
                }
            }
        }

        let min_axis = min_axis.unwrap().clone();
        let a_projection = a_position.dot(&min_axis);
        let b_projection = b_position.dot(&min_axis);

        let mut collision_normal = min_axis;

        if b_projection - a_projection < 0.0 {
            collision_normal *= -1.0;
        }

        Some(CollisionData {
            bodies: [a_id, b_id],
            // Set point
            point: Vector::zeros(),
            normal: collision_normal,
            depth: min_penetration,
        })
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
        restitutions: &HashMap<Id, f64>,
        shapes: &HashMap<Id, Shape>,
    ) {
        for collision in collisions {
            let a_id = collision.bodies[0];
            let b_id = collision.bodies[1];
            let [a_linear, b_linear] = linear_states.get_disjoint_mut([&a_id, &b_id]);
            let a_linear = a_linear.unwrap();
            let b_linear = b_linear.unwrap();
            let [a_restitution, b_restitution] = [
                restitutions.get(&a_id).unwrap(),
                restitutions.get(&b_id).unwrap(),
            ];

            let impulse = -(1.0 + *a_restitution * *b_restitution)
                * (b_linear.velocity - a_linear.velocity).dot(&collision.normal)
                / (1.0 / a_linear.mass + 1.0 / b_linear.mass);

            a_linear.velocity -= impulse / a_linear.mass * collision.normal;
            b_linear.velocity += impulse / b_linear.mass * collision.normal;

            // Positional correction
            if collision.depth > self.correction_tolerance {
                let correction = (collision.depth * self.correction_level * collision.normal)
                    / (1.0 / a_linear.mass + 1.0 / b_linear.mass);
                a_linear.position -= correction / a_linear.mass;
                b_linear.position += correction / b_linear.mass;
            }
        }
    }
}
