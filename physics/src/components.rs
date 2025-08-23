use crate::body::{AngularState, LinearState, Shape};

pub struct Components {
    linear: Option<LinearState>,
    angular: Option<AngularState>,
    shape: Option<Shape>,
}
