use std::collections::HashMap;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use speedy2d::shape::Rect;
use speedy2d::window::{KeyScancode, VirtualKeyCode};
use crate::modules::node_container::node_container::STATE::*;
use crate::structs::module::Module;


const IDLE_CURSOR_MOVE_SPEED: f32 = 230.0;


enum STATE {
    IDLE(Vec2, (f32, f32, f32, f32)), // (cursor_position: Vec2, cursor_speed: up, down, left, right)
}

pub struct NodeContainer {
    state: STATE,
    viewport: Rect,
    pressed_keys: Vec<VirtualKeyCode>,
}

impl NodeContainer {
    pub fn new() -> Self {
        Self {
            state: STATE::IDLE(Vec2::new(0.0, 0.0), (0.0, 0.0, 0.0, 0.0)),
            viewport: Rect::from_tuples((0.0, 0.0), (0.0, 0.0)),
            pressed_keys: Vec::new(),
        }
    }
}

impl Module for NodeContainer {
    
    fn get_name(&self) -> String {
        "NodeContainer".to_string()
    }

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect, delta_time: f64) {

        if self.viewport.is_zero_area() {
            self.viewport = viewport.clone();
            self.state = IDLE(Vec2::new(self.viewport.width() / 2.0 + viewport.left(), self.viewport.height() / 2.0 + viewport.top()), (0.0, 0.0, 0.0, 0.0));
        }

        match &mut self.state {
            IDLE(position, mouse_speed) => {

                //move mouse
                position.y += mouse_speed.0 * delta_time as f32;
                position.x += mouse_speed.1 * delta_time as f32;
                position.y += mouse_speed.2 * delta_time as f32;
                position.x += mouse_speed.3 * delta_time as f32;

                graphics.draw_circle(position, 10.0, speedy2d::color::Color::RED);
            }
        }

    }

    fn handle_key_down(&mut self, key: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        if let Some(key) = key {
            match self.state {
                IDLE(_, ref mut speed) => {
                    match key {
                        VirtualKeyCode::I => speed.0 = -IDLE_CURSOR_MOVE_SPEED,
                        VirtualKeyCode::J => speed.1 = -IDLE_CURSOR_MOVE_SPEED,
                        VirtualKeyCode::K => speed.2 = IDLE_CURSOR_MOVE_SPEED,
                        VirtualKeyCode::O => speed.3 = IDLE_CURSOR_MOVE_SPEED,
                        _ => {}
                    }
                }
            }
        }
    }

    fn handle_key_up(&mut self, key: Option<VirtualKeyCode>, scancode: KeyScancode) {
        if let Some(key) = key {
            match self.state {
                IDLE(_, ref mut speed) => {
                    match key {
                        VirtualKeyCode::I => speed.0 = 0.0,
                        VirtualKeyCode::J => speed.1 = 0.0,
                        VirtualKeyCode::K => speed.2 = 0.0,
                        VirtualKeyCode::O => speed.3 = 0.0,
                        _ => {}
                    }
                }
            }
        }
    }

}