use speedy2d::dimen::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    viewport_position: Vec2,
    module_position: Vec2,
}

impl MousePosition {
    pub fn new(viewport_position: Vec2, module_position: Vec2) -> MousePosition {
        MousePosition {
            viewport_position,
            module_position,
        }
    }

    pub fn viewport(&self) -> Vec2 {
        self.viewport_position
    }
    pub fn module(&self) -> Vec2 {
        self.module_position
    }
}
