use resource::ResourceMap;

pub mod resource;

#[derive(Default)]
pub struct World {
    pub resources: ResourceMap,
}

impl World {
    pub fn new() -> Self {
        Default::default()
    }
}
