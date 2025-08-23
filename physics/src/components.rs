use crate::body::{AngularState, LinearState, Shape};

pub struct Components {
    pub linear: Option<LinearState>,
    pub angular: Option<AngularState>,
    pub shape: Option<Shape>,
}

impl Default for Components {
    fn default() -> Self {
        Self {
            linear: None,
            angular: None,
            shape: None,
        }
    }
}
