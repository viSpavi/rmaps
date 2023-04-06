use std::fs::File;
use std::iter::Map;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use speedy2d::shape::Rect;
use speedy2d::window::{MouseScrollDistance, VirtualKeyCode};
use crate::structs::mouse_position::MousePosition;

pub trait Module {

    fn load(&mut self);
    fn unload(&mut self);

    fn get_name(&self) -> String;

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect);

    fn open(&mut self);
    fn close(&mut self);

    fn get_active_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()>;
    fn get_persistent_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()>;

    fn handle_click(&mut self, position: MousePosition, click_count: i32);

    fn handle_drag(&mut self, position: MousePosition, distance: MouseScrollDistance) {
        // do nothing
    }

}