use crate::Layer;

pub struct Engine {
    #[allow(unused)]
    layers: Vec<Box<dyn Layer>>,
}
