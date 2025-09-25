use std::any::Any;

pub mod math {
    pub type Vector<T> = nalgebra::Vector2<T>;
    pub type Point<T> = nalgebra::Point2<T>;
    pub type Translation<T> = nalgebra::Translation2<T>;
    pub type Rotation<T> = nalgebra::Rotation2<T>;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
