use std::f64;

use crate::{collision::*, id_map::Id};

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
    fn init(&mut self, bodies: &mut IdMap<Body>) {
        self.detector.init(&self.bodies, bodies);
    }

    fn handle(&mut self, bodies: &mut IdMap<Body>) {
        let collisions = self.detector.detect(&self.bodies, bodies);
        self.resolver.resolve(collisions, bodies);
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
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) {
        self.broad_phase.init(managed_bodies, bodies);
    }

    fn detect(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) -> Vec<CollisionData> {
        let body_pairs = self.broad_phase.cull(managed_bodies, bodies);
        self.narrow_phase.detect(body_pairs, bodies)
    }
}

pub struct DefaultBroadPhase {
    circles: HashMap<Id, f64>,
}

impl DefaultBroadPhase {
    pub fn new() -> Self {
        Self {
            circles: HashMap::new(),
        }
    }
}

impl BroadPhase for DefaultBroadPhase {
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) {
        for id in managed_bodies {
            if let Some(body) = bodies.get(*id) {
                match &body.shape {
                    Shape::Point => {
                        self.circles.insert(*id, 0.0);
                    }
                    Shape::Circle(radius) => {
                        self.circles.insert(*id, *radius);
                    }
                    Shape::Rectangle(size) => {
                        let radius = ((size.x / 2.0).powi(2) + (size.y / 2.0).powi(2)).sqrt();
                        self.circles.insert(*id, radius);
                    }
                    Shape::Polygon(points) => {
                        let mut max_radius: f64 = 0.0;

                        for point in points {
                            max_radius = max_radius.max(point.magnitude());
                        }

                        self.circles.insert(*id, max_radius);
                    }
                }
            }
        }
    }

    fn cull(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) -> Vec<[Id; 2]> {
        let mut pairs = Vec::new();

        for i in 0..managed_bodies.len() {
            for j in (i + 1)..managed_bodies.len() {
                let (a_id, b_id) = (managed_bodies[i], managed_bodies[j]);
                let Some(a) = bodies.get(a_id) else { continue };
                let Some(b) = bodies.get(b_id) else { continue };

                let distance = b.linear.position.metric_distance(&a.linear.position);

                if distance < self.circles.get(&a_id).unwrap() + self.circles.get(&b_id).unwrap() {
                    pairs.push([a_id, b_id]);
                }
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
    fn init(&mut self, managed_bodies: &Vec<Id>, bodies: &mut IdMap<Body>) {}

    fn detect(&mut self, body_pairs: Vec<[Id; 2]>, bodies: &mut IdMap<Body>) -> Vec<CollisionData> {
        let mut collisions = Vec::new();

        for pair in body_pairs {
            let (a_id, b_id) = (pair[0], pair[1]);
            let Some(a) = bodies.get(a_id) else { continue };
            let Some(b) = bodies.get(b_id) else { continue };

            if let Some(collision) = match a.shape {
                Shape::Circle(a_radius) => match b.shape {
                    Shape::Circle(b_radius) => DefaultNarrowPhase::detect_circle_circle(
                        pair[0],
                        &a.linear.position,
                        a_radius,
                        pair[1],
                        &b.linear.position,
                        b_radius,
                    ),
                    _ => None,
                },
                Shape::Rectangle(a_size) => match b.shape {
                    Shape::Rectangle(b_size) => DefaultNarrowPhase::detect_sat(
                        pair[0],
                        &a.linear.position,
                        vec![
                            b.linear.position
                                + Rotation::new(-b.angular.rotation)
                                    * Vector::new(-b_size.x / 2.0, b_size.y / 2.0),
                            b.linear.position
                                + Rotation::new(-b.angular.rotation)
                                    * Vector::new(b_size.x / 2.0, b_size.y / 2.0),
                            b.linear.position
                                + Rotation::new(-b.angular.rotation)
                                    * Vector::new(b_size.x / 2.0, -b_size.y / 2.0),
                            b.linear.position
                                + Rotation::new(-b.angular.rotation)
                                    * Vector::new(-b_size.x / 2.0, -b_size.y / 2.0),
                        ],
                        vec![Vector::new(0.0, 1.0), Vector::new(1.0, 0.0)],
                        a.angular.rotation,
                        pair[1],
                        &b.linear.position,
                        vec![
                            a.linear.position
                                + Rotation::new(-a.angular.rotation)
                                    * Vector::new(-a_size.x / 2.0, a_size.y / 2.0),
                            a.linear.position
                                + Rotation::new(-a.angular.rotation)
                                    * Vector::new(a_size.x / 2.0, a_size.y / 2.0),
                            a.linear.position
                                + Rotation::new(-a.angular.rotation)
                                    * Vector::new(a_size.x / 2.0, -a_size.y / 2.0),
                            a.linear.position
                                + Rotation::new(-a.angular.rotation)
                                    * Vector::new(-a_size.x / 2.0, -a_size.y / 2.0),
                        ],
                        vec![Vector::new(0.0, 1.0), Vector::new(1.0, 0.0)],
                        b.angular.rotation,
                    ),
                    _ => None,
                },
                // Shape::Polygon(a_points) => match b_shape {
                //     Shape::Polygon(b_points) => DefaultNarrowPhase::detect_sat(
                //         pair[0],
                //         pair[1],
                //         a_points,
                //         b_points,
                //         a.linear.position,
                //         b.linear.position,
                //         a_normals,
                //         b_normals,
                //         a.angular.rotation,
                //         b.angular.rotation,
                //     ),
                //     _ => None,
                // },
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
        a_position: &Vector<f64>,
        a_radius: f64,
        b_id: Id,
        b_position: &Vector<f64>,
        b_radius: f64,
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
        a_position: &Vector<f64>,
        a_points: Vec<Vector<f64>>,
        a_normals: Vec<Vector<f64>>,
        a_rotation: f64,
        b_id: Id,
        b_position: &Vector<f64>,
        b_points: Vec<Vector<f64>>,
        b_normals: Vec<Vector<f64>>,
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
        let mut a_closest_point_index: usize;
        let mut b_closest_point_index: usize;

        for axis in &axes {
            let mut a_min = f64::INFINITY;
            let mut a_max = f64::NEG_INFINITY;
            let mut b_min = f64::INFINITY;
            let mut b_max = f64::NEG_INFINITY;

            // Project a points
            for point in &a_points {
                let projection = point.dot(axis);
                a_min = a_min.min(projection);
                a_max = a_max.max(projection);
            }

            // Project b points
            for point in &b_points {
                let projection = point.dot(axis);
                b_min = b_min.min(projection);
                b_max = b_max.max(projection);
            }

            // Calculate penetration depth
            let penetration = a_max.min(b_max) - a_min.max(b_min);

            if b_min >= a_max || a_min >= b_max {
                // Axis has no intersection so no collision
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

    pub fn detect_sat_circle(
        a_id: Id,
        a_points: Vec<Vector<f64>>,
        a_position: &Vector<f64>,
        a_normals: Vec<Vector<f64>>,
        a_rotation: f64,
        b_id: Id,
        b_position: &Vector<f64>,
        b_radius: f64,
    ) -> Option<CollisionData> {
        // Pass in axes maybe
        let mut axes: Vec<Vector<f64>> = Vec::new();

        for normal in a_normals {
            axes.push(Rotation::new(-a_rotation) * normal);
        }

        let mut min_penetration = f64::INFINITY;
        let mut min_axis: Option<&Vector<f64>> = None;

        for axis in &axes {
            let mut a_min = f64::INFINITY;
            let mut a_max = f64::NEG_INFINITY;

            // Project a points
            for point in &a_points {
                let projection = point.dot(axis);
                a_min = a_min.min(projection);
                a_max = a_max.max(projection);
            }

            // Project b points
            let b_min = b_position.dot(axis) - b_radius;
            let b_max = b_position.dot(axis) + b_radius;

            // Calculate penetration depth
            let penetration = a_max.min(b_max) - a_min.max(b_min);

            if b_min >= a_max || a_min >= b_max {
                // Axis has no intersection so no collision
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

        let collision_point = b_position + collision_normal * min_penetration;

        Some(CollisionData {
            bodies: [a_id, b_id],
            point: collision_point,
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
    fn init(&mut self, bodies: &mut IdMap<Body>) {}

    fn resolve(&mut self, collisions: Vec<CollisionData>, bodies: &mut IdMap<Body>) {
        for collision in collisions {
            // MAKE A GET DISJOINT MUT FOR BODIES
            let (a_id, b_id) = (collision.bodies[0], collision.bodies[1]);
            let Some(a) = bodies.get(a_id) else { continue };
            let Some(b) = bodies.get(b_id) else { continue };

            let impulse = -(1.0 + a.restitution * b.restitution)
                * (b.linear.velocity - a.linear.velocity).dot(&collision.normal)
                / (1.0 / a.linear.mass + 1.0 / b.linear.mass);

            let Some(a) = bodies.get_mut(a_id) else {
                continue;
            };
            a.linear.velocity -= impulse / a.linear.mass * collision.normal;
            let Some(b) = bodies.get_mut(b_id) else {
                continue;
            };
            b.linear.velocity += impulse / b.linear.mass * collision.normal;

            // Positional correction
            if collision.depth > self.correction_tolerance {
                let Some(a) = bodies.get(a_id) else { continue };
                let Some(b) = bodies.get(b_id) else { continue };
                let correction = (collision.depth * self.correction_level * collision.normal)
                    / (1.0 / a.linear.mass + 1.0 / b.linear.mass);

                let Some(a) = bodies.get_mut(a_id) else {
                    continue;
                };
                a.linear.position -= correction / a.linear.mass;
                let Some(b) = bodies.get_mut(b_id) else {
                    continue;
                };
                b.linear.position += correction / b.linear.mass;
            }
        }
    }
}
