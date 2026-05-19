use crate::objects::Object;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![],
        }
    }

    pub fn add_object(&mut self, object: impl Into<Object>) {
        self.objects.push(object.into());
    }
}
