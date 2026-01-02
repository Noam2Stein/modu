use crate::{Ecs, Params, Systems};

#[derive(Debug, Clone, Default)]
pub struct Schedule {
    _data: (),
}

impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_systems(&mut self, systems: impl Systems) {
        let _ = systems;
        todo!()
    }

    pub fn run(&self, ecs: &mut Ecs) {
        self.run_with(ecs, ());
    }

    pub fn run_with(&self, ecs: &mut Ecs, params: impl Params) {
        let _ = ecs;
        let _ = params;
        todo!()
    }
}
