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
                    // Shape::Rectangle(b_size) => DefaultNarrowPhase::detect_rectangle_rectangle(
                    //     pair[0],
                    //     pair[1],
                    //     a_size,
                    //     b_size,
                    //     // Reference or not
                    //     &a_linear.position,
                    //     &b_linear.position,
                    //     a_angular.rotation,
                    //     b_angular.rotation,
                    // ),
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
        b_id: Id,
        a_radius: f64,
        b_radius: f64,
        a_position: &Vector<f64>,
        b_position: &Vector<f64>,
    ) -> Option<CollisionData> {
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

    // Separating Axis Theorem (SAT)
    pub fn detect_sat(
        a_id: Id,
        b_id: Id,
        a_points: Vec<Vector<f64>>,
        b_points: Vec<Vector<f64>>,
    ) -> Option<CollisionData> {
        let mut axes: Vec<Vector<f64>> = Vec::new();

        for i in 0..a_points.len() {
            // should be perpendicular to points
            let axis = (a_points[i] - a_points[(i + 1) % a_points.len()]).normalize();

            // use constant
            // only use for big polygons?
            if axes.iter().any(|a| axis.dot(a).abs() > 0.999) {
                continue;
            }

            axes.push(axis);
        }

        for i in 0..b_points.len() {
            axes.push((b_points[i] - b_points[i + 1 % b_points.len()]).normalize());
        }

        // for i in axes.len() {
        //     (a_position + Rotation::new(a_rotation) * a_points[i])
        // }
        // let a_min;
        // let a_max;
        // let projected = point.dot(&axis);
        // println!("{}", projected);

        None
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
