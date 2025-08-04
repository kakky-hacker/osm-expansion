use super::Joint;
use std::rc::Rc;
pub struct Link {
    pub lanes: Option<usize>,
    pub width: Option<f32>,
    pub joints: Vec<Rc<Joint>>,
}

impl Link {
    pub fn new(lanes: Option<usize>, width: Option<f32>, joints: Vec<Rc<Joint>>) -> Self {
        Link {
            lanes,
            width,
            joints,
        }
    }
}
