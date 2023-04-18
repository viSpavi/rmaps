use crate::structs::module::Module;
use crate::structs::mouse_position::MousePosition;
use speedy2d::color::Color;
use speedy2d::shape::{Rect};
use speedy2d::window::VirtualKeyCode;
use speedy2d::Graphics2D;
use std::collections::HashMap;

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
    fn unload(&mut self) {}

    fn load(&mut self) {}

    fn get_name(&self) -> String {
        "Dummy Module".to_string()
    }

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect, _delta_time: f64) {
        graphics.draw_rectangle(viewport, Color::BLUE);
    }

    fn open(&mut self) {
        println!("loaded dummy module");
    }

    fn close(&mut self) {
        println!("unloaded dummy module");
    }

    fn get_active_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        todo!()
    }

    fn get_persistent_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        todo!()
    }
}
