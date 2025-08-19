use crate::collision::*;

pub struct DefaultCollisionDetector {
    broad_phase: DefaultBroadPhase,
    narrow_phase: DefaultNarrowPhase,
}

impl CollisionDetection for DefaultCollisionDetector {
    fn new() -> Self {
        Self {
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: DefaultNarrowPhase::new(),
        }
    }

    fn detect(
        &mut self,
        objects: Vec<Id>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        self.narrow_phase
            .detect(self.broad_phase.cull(objects), linear_states, shapes)
    }
}

pub struct DefaultCollisionResolver {}

impl CollisionResolution for DefaultCollisionResolver {
    fn new() -> Self {
        Self {}
    }

    fn resolve(
        &mut self,
        collisions: Vec<CollisionData>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) {
    }
}

pub struct DefaultBroadPhase {}

impl BroadPhase for DefaultBroadPhase {
    fn new() -> Self {
        Self {}
    }

    fn cull(&mut self, objects: Vec<Id>) -> Vec<[Id; 2]> {
        let mut pairs = Vec::new();

        for i in 0..objects.len() {
            for j in (i + 1)..objects.len() {
                pairs.push([objects[i], objects[j]]);
            }
        }

        pairs
    }
}

pub struct DefaultNarrowPhase {}

impl NarrowPhase for DefaultNarrowPhase {
    fn new() -> Self {
        Self {}
    }

    fn detect(
        &mut self,
        object_pairs: Vec<[Id; 2]>,
        linear_states: &HashMap<Id, LinearState>,
        shapes: &HashMap<Id, Shape>,
    ) -> Vec<CollisionData> {
        let mut collisions = Vec::new();

        for pair in object_pairs {
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
        let distance = a_position.metric_distance(&b_position);
        let depth = a_radius + b_radius - distance;
        let normal = (a_position - b_position).normalize();

        if depth > 0.0 {
            Some(CollisionData {
                objects: [a_id, b_id],
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
