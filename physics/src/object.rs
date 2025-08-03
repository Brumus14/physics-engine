use nalgebra::Vector2;

pub enum Shape {
    Point,
    Circle(f64),
    Polygon(Vec<Vector2<f64>>),
}

pub struct Object {
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
    // Add unit conversions
    pub mass: f64,
    pub shape: Shape,
}

impl Object {
    pub fn new(
        position: Vector2<f64>,
        velocity: Vector2<f64>,
        acceleration: Vector2<f64>,
        mass: f64,
        shape: Shape,
    ) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            mass,
            shape,
        }
    }

    pub fn step(&mut self, delta_time: f64) {
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }
}
