pub mod circle_collision;
pub mod collision_spring;
pub mod falling_circles;
pub mod falling_rectangles;
pub mod orbit;
pub mod polygon;
pub mod spring;
pub mod tower;

#[derive(Clone, Copy)]
pub enum PhysicsScene {
    FallingRectangles,
    FallingCircles,
    Tower,
    CircleCollision,
    Spring,
    CollisionSpring,
    Polygon,
    Orbit,
}
