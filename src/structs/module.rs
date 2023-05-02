use crate::structs::mouse_position::MousePosition;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;
use speedy2d::window::{KeyScancode, MouseButton, MouseScrollDistance, VirtualKeyCode};
use speedy2d::Graphics2D;
use std::collections::HashMap;
use std::fs::File;
use std::iter::Map;

pub trait Module {

    fn load(&mut self) {
        println!("Module {} loaded", self.get_name());
    }
    fn unload(&mut self) {
        println!("Module {} unloaded", self.get_name());
    }

    fn get_name(&self) -> String;

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect, delta_time: f64);

    fn open(&mut self) {}
    fn close(&mut self) {}

    fn get_active_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        HashMap::new()
    }
    fn get_persistent_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        HashMap::new()
    }

    fn handle_mouse_down(&mut self, _position: MousePosition, _click_count: i32, _button: MouseButton) {
        // do nothing
    }

    fn handle_mouse_up(&mut self, _position: MousePosition, _click_count: i32, _button: MouseButton) {
        // do nothing
    }

    fn handle_mouse_move(&mut self, _position: MousePosition) {
        // do nothing
    }

    fn handle_drag(&mut self, _position: MousePosition, _distance: MouseScrollDistance) {
        // do nothing
    }

    fn handle_key_down(&mut self, _key: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        // do nothing
    }

    fn handle_key_up(&mut self, _key: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        // do nothing
    }

    fn handle_char(&mut self, _character: char) {
        // do nothing
    }

}
