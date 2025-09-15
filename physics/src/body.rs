use std::f64;

use crate::{effector::Spring, types::math::*};

#[derive(Clone)]
pub struct Body {
    // Not pub? add getters?
    pub linear: LinearState,
    // Only for collidable
    pub restitution: f64,
    // Both optional then remove point shape?
    pub angular: AngularState,
    pub shape: Shape,
}

// Maybe add function to apply force at a point
impl Body {
    // Rename to just new?
    pub fn new_rigid(
        linear: LinearState,
        restitution: f64,
        angular: AngularState,
        shape: Shape,
    ) -> Self {
        Self {
            linear,
            restitution,
            angular,
            shape,
        }
    }

    pub fn new_particle(linear: LinearState, restitution: f64) -> Self {
        Self {
            linear,
            restitution,
            // Seems cheaty
            angular: AngularState::new(0.0, 0.0, 0.0),
            shape: Shape::Point,
        }
    }
}

#[derive(Clone)]
pub struct LinearState {
    pub position: Vector<f64>,
    pub velocity: Vector<f64>,
    pub force: Vector<f64>,
    pub mass: f64,
}

impl LinearState {
    pub fn new(position: Vector<f64>, velocity: Vector<f64>, mass: f64) -> Self {
        Self {
            position,
            velocity,
            force: Vector::zeros(),
            mass,
        }
    }
}

#[derive(Clone)]
// Counter clockwise is positive
pub struct AngularState {
    pub orientation: f64,
    pub velocity: f64,
    pub torque: f64,
    pub inertia: f64,
}

impl AngularState {
    pub fn new(orientation: f64, velocity: f64, inertia: f64) -> Self {
        Self {
            orientation,
            velocity,
            torque: 0.0,
            inertia,
        }
    }
}

#[derive(Clone)]
pub enum Shape {
    Point,
    Circle(f64),
    Polygon {
        points: Vec<Vector<f64>>,
        axes: Vec<Vector<f64>>,
    },
}

impl Shape {
    pub fn new_circle(radius: f64) -> Self {
        Shape::Circle(radius)
    }

    pub fn new_rectangle(size: Vector<f64>) -> Self {
        let half_size = size / 2.0;
        Shape::Polygon {
            points: vec![
                Vector::new(half_size.x, half_size.y),
                Vector::new(-half_size.x, half_size.y),
                Vector::new(-half_size.x, -half_size.y),
                Vector::new(half_size.x, -half_size.y),
            ],
            axes: vec![Vector::new(1.0, 0.0), Vector::new(0.0, 1.0)],
        }
    }

    // Counter clockwise points
    pub fn new_polygon(points: Vec<Vector<f64>>) -> Self {
        let mut axes: Vec<Vector<f64>> = Vec::new();

        for i in 0..points.len() {
            let line = (points[(i + 1) % points.len()] - points[i]).normalize();
            let normal = Vector::new(line.y, -line.x);

            // let duplicate_axis = axes.iter().any(|a| {
            //     // Move to const
            //     normal.dot(a).abs() > 0.999
            // });
            //
            // if !duplicate_axis {
            axes.push(normal);
            // }
        }

        Shape::Polygon { points, axes }
    }

    pub fn project(
        &self,
        axis: &Vector<f64>,
        position: &Vector<f64>,
        orientation: f64,
    ) -> (f64, f64) {
        // Account for axis rotation
        match self {
            Shape::Point => {
                let projection = position.dot(axis);
                (projection, projection)
            }
            Shape::Circle(radius) => {
                let projection = position.dot(axis);
                (projection - radius, projection + radius)
            }
            Shape::Polygon { points, axes: _ } => {
                let mut min = f64::INFINITY;
                let mut max = f64::NEG_INFINITY;

                for point in points {
                    let projection = (position + Rotation::new(orientation) * point).dot(axis);
                    min = min.min(projection);
                    max = max.max(projection);
                }

                (min, max)
            }
        }
    }

    // Is edge the best name, most perpendicular
    pub fn farthest_perpendicular_edge(
        &self,
        axis: &Vector<f64>,
        position: &Vector<f64>,
        orientation: f64,
    ) -> (Vector<f64>, Vector<f64>) {
        match self {
            Shape::Point => (*position, *position),
            Shape::Circle(radius) => {
                let point = position + *axis * *radius;
                (point, point)
            }
            Shape::Polygon { points, axes: _ } => {
                let mut max = f64::NEG_INFINITY;
                let mut max_point_index = 0;

                let global_points: Vec<Vector<f64>> = points
                    .iter()
                    .map(|p| position + Rotation::new(orientation) * p)
                    .collect();

                for i in 0..global_points.len() {
                    let projection = global_points[i].dot(axis);

                    if projection > max {
                        max = projection;
                        max_point_index = i;
                    }
                }

                let point = global_points[max_point_index];
                let (a, b) = (
                    global_points
                        [(max_point_index + global_points.len() - 1) % global_points.len()],
                    global_points[(max_point_index + 1) % global_points.len()],
                );

                let a_edge = (point, a);
                let b_edge = (point, b);

                if (point - a).normalize().dot(&axis).abs()
                    <= (point - b).normalize().dot(&axis).abs()
                {
                    a_edge
                } else {
                    b_edge
                }
            }
        }
    }
}
