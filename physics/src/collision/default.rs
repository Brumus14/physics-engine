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
                    Shape::Polygon { points, axes: _ } => {
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

    // Doesnt work for points
    fn detect(&mut self, body_pairs: Vec<[Id; 2]>, bodies: &mut IdMap<Body>) -> Vec<CollisionData> {
        let mut collisions = Vec::new();

        for pair in body_pairs {
            let (a_id, b_id) = (pair[0], pair[1]);
            let Some(a) = bodies.get(a_id) else { continue };
            let Some(b) = bodies.get(b_id) else { continue };

            let collision = match (&a.shape, &b.shape) {
                // How to handle points?
                (Shape::Point, Shape::Point) => None,
                (Shape::Point, Shape::Circle(b_radius)) => {
                    DefaultNarrowPhase::detect_circle_circle(
                        pair[0],
                        &a.linear.position,
                        0.0,
                        pair[1],
                        &b.linear.position,
                        *b_radius,
                    )
                }
                (Shape::Circle(a_radius), Shape::Point) => {
                    DefaultNarrowPhase::detect_circle_circle(
                        pair[0],
                        &a.linear.position,
                        *a_radius,
                        pair[1],
                        &b.linear.position,
                        0.0,
                    )
                }
                (Shape::Circle(a_radius), Shape::Circle(b_radius)) => {
                    DefaultNarrowPhase::detect_circle_circle(
                        pair[0],
                        &a.linear.position,
                        *a_radius,
                        pair[1],
                        &b.linear.position,
                        *b_radius,
                    )
                }
                (
                    Shape::Polygon {
                        points: a_points,
                        axes: a_axes,
                    },
                    Shape::Polygon {
                        points: b_points,
                        axes: b_axes,
                    },
                ) => DefaultNarrowPhase::detect_sat(
                    pair[0],
                    &a.linear.position,
                    a_points,
                    a_axes,
                    pair[1],
                    &b.linear.position,
                    b_points,
                    b_axes,
                ),
                _ => None,
            };

            if let Some(collision) = collision {
                self.collisions += 1;
                println!("{:?}", collision);
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

        let a_point = a_position + normal * a_radius;
        let b_point = b_position - normal * b_radius;
        // Make separate point function
        let point = (a_point * a_radius + b_point * b_radius) / (a_radius + b_radius);

        if depth > 0.0 {
            Some(CollisionData {
                bodies: [a_id, b_id],
                point,
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
    // Does / should this be collision when touching?
    // Takes global points
    fn detect_sat(
        a_id: Id,
        a_position: &Vector<f64>,
        a_points: &Vec<Vector<f64>>,
        a_axes: &Vec<Vector<f64>>,
        b_id: Id,
        b_position: &Vector<f64>,
        b_points: &Vec<Vector<f64>>,
        b_axes: &Vec<Vector<f64>>,
    ) -> Option<CollisionData> {
        // Pass in axes maybe
        let mut axes: Vec<Vector<f64>> = Vec::new();

        // Dont add duplicate axes
        for axis in a_axes {
            axes.push(*axis);
        }

        for axis in b_axes {
            axes.push(-*axis);
        }

        let mut min_penetration = f64::INFINITY;
        let mut min_axis: Option<&Vector<f64>> = None;

        for axis in &axes {
            let (a_min, a_max) = DefaultNarrowPhase::project(a_points, axis);
            let (b_min, b_max) = DefaultNarrowPhase::project(b_points, axis);

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

        let a_edge = DefaultNarrowPhase::farthest_perpendicular_edge(a_points, &min_axis);
        let b_edge = DefaultNarrowPhase::farthest_perpendicular_edge(b_points, &-min_axis);

        let (reference, incident) = if (a_edge.0 - a_edge.1)
            .normalize()
            .dot(&collision_normal)
            .abs()
            <= (b_edge.0 - b_edge.1)
                .normalize()
                .dot(&collision_normal)
                .abs()
        {
            (a_edge, b_edge)
        } else {
            (b_edge, a_edge)
        };

        Some(CollisionData {
            bodies: [a_id, b_id],
            // Set point
            point: *collision_point,
            normal: collision_normal,
            depth: min_penetration,
        })
    }

    fn project(points: &Vec<Vector<f64>>, axis: &Vector<f64>) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for point in points {
            let projection = point.dot(axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }

    fn farthest_perpendicular_edge(
        points: &Vec<Vector<f64>>,
        axis: &Vector<f64>,
    ) -> (Vector<f64>, Vector<f64>) {
        let mut max = f64::NEG_INFINITY;
        let mut max_point_index = 0;

        for i in 0..points.len() {
            let projection = points[i].dot(axis);

            if projection > max {
                max = projection;
                max_point_index = i;
            }
        }

        let point = points[max_point_index];
        let (a, b) = (
            points[(max_point_index + points.len() - 1) % points.len()],
            points[(max_point_index + 1) % points.len()],
        );

        let a_edge = (point, a);
        let b_edge = (point, b);

        if (point - a).normalize().dot(&axis).abs() <= (point - b).normalize().dot(&axis).abs() {
            a_edge
        } else {
            b_edge
        }
    }

    fn clip(
        edge: (Vector<f64>, Vector<f64>),
        normal: &Vector<f64>,
        offset: f64,
    ) -> Vec<Vector<f64>> {
        let mut points = Vec::new();

        let a = edge.0.dot(normal) - offset;
        let b = edge.1.dot(normal) - offset;

        if a >= 0.0 {
            points.push(edge.0);
        }

        if b >= 0.0 {
            points.push(edge.1);
        }

        if a * b < 0.0 {
            points.push((edge.1 - edge.0) * (a / (a - b)) + edge.0);
        }

        points
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
            // Make a get disjoint mut for bodies
            let (a_id, b_id) = (collision.bodies[0], collision.bodies[1]);
            let Some(a) = bodies.get(a_id) else { continue };
            let Some(b) = bodies.get(b_id) else { continue };

            let restitution = a.restitution * b.restitution;

            let a_to_point = collision.point - a.linear.position;
            let a_to_point_perp = Vector::new(-a_to_point.y, a_to_point.x);
            let b_to_point = collision.point - b.linear.position;
            let b_to_point_perp = Vector::new(-b_to_point.y, b_to_point.x);

            let relative_velocity = (b.linear.velocity + b.angular.velocity * b_to_point_perp)
                - (a.linear.velocity + a.angular.velocity * a_to_point_perp);

            // Separate calculation for none angular bodies
            // What happens if one can rotate but other cant?
            let impulse_magnitude = -(1.0 + restitution) * relative_velocity.dot(&collision.normal)
                / (1.0 / a.linear.mass
                    + 1.0 / b.linear.mass
                    + a_to_point.perp(&collision.normal).powi(2) / a.angular.inertia
                    + b_to_point.perp(&collision.normal).powi(2) / b.angular.inertia);

            let Some(a) = bodies.get_mut(a_id) else {
                continue;
            };
            a.linear.velocity -= impulse_magnitude / a.linear.mass * collision.normal;
            a.angular.velocity -=
                a_to_point.perp(&(impulse_magnitude * collision.normal)) / a.angular.inertia;

            let Some(b) = bodies.get_mut(b_id) else {
                continue;
            };
            b.linear.velocity += impulse_magnitude / b.linear.mass * collision.normal;
            b.angular.velocity +=
                b_to_point.perp(&(impulse_magnitude * collision.normal)) / b.angular.inertia;

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
