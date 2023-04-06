use std::fs::File;
use std::io::Write;
use std::iter::Map;
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;
use speedy2d::shape::{Rect, Rectangle};
use speedy2d::window::VirtualKeyCode;
use crate::structs::module::Module;
use crate::structs::mouse_position::MousePosition;

pub struct DummyModule {
    // ...
}

impl DummyModule {
    pub(crate) fn new() -> DummyModule {
        DummyModule {
            // ...
        }
    }
}

impl Module for DummyModule {
    fn unload(&mut self) {

    }

    fn load(&mut self) {

    }

    fn get_name(&self) -> String {
        "Dummy Module".to_string()
    }

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect) {
        graphics.draw_rectangle(viewport, Color::BLUE);
    }

    fn open(&mut self) {
        println!("loaded dummy module");
    }

    fn close(&mut self) {
        println!("unloaded dummy module");
    }

    fn get_active_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()> {
        todo!()
    }

    fn get_persistent_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()> {
        todo!()
    }

    fn handle_click(&mut self, position: MousePosition, click_count: i32) {
        println!("clicked dummy module at {:?}", position.viewport());
    }
}