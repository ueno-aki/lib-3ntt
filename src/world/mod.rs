use resource::ResourceMap;

pub mod resource;

#[derive(Default)]
pub struct World {
    resources: ResourceMap,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}
