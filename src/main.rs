mod structs;
mod modules;
mod types;

use std::cell::RefCell;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{Deref, Sub};
use std::rc::Rc;
use speedy2d::{Graphics2D, Window};
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2, Vector2};
use speedy2d::shape::{Rect, Rectangle};
use speedy2d::window::{MouseButton, MouseScrollDistance, WindowHandler, WindowHelper};
use crate::modules::*;
use crate::structs::module::Module;
use crate::structs::node::{Node, NODE_COUNTER};
use crate::structs::link::Link;
use std::fs::OpenOptions;
use std::process::id;
use std::sync::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use lazy_static::lazy_static;
use crate::modules::dummy::DummyModule;
use crate::modules::generic_node_container::GenericNodeContainer;
use crate::modules::side_panel::SidePanel;
use crate::modules::top_panel::TopPanel;
use crate::structs::mouse_position::MousePosition;
use crate::types::DoublePointerSafe;

lazy_static! {
    static ref NODES: Arc<RwLock<Vec<Arc<RwLock<Node>>>>> = Arc::new(RwLock::new(Vec::new()));
    static ref LINKS: Arc<RwLock<Vec<Arc<RwLock<Link>>>>> = Arc::new(RwLock::new(Vec::new()));
    static ref MODULES: Arc<RwLock<Vec<Arc<RwLock<Box<dyn Module+Send+Sync>>>>>> = {
        let mut modules: Vec<Arc<RwLock<Box<dyn Module+Send+Sync>>>> = vec![
            Arc::new(RwLock::new(Box::new(GenericNodeContainer::new()))),
            Arc::new(RwLock::new(Box::new(DummyModule::new())))
        ];

        modules.iter_mut().for_each(|module| {
            module.write().unwrap().load();
        });
        Arc::new(RwLock::new(modules))
    };
    //double pointer. The main one (first arc/rwlock) is the static one while we modify the second one by replacing it with pointers to other modules.
    static ref ACTIVE_MODULE: DoublePointerSafe<Box<dyn Module+Send+Sync>> = Arc::new(RwLock::new(MODULES.read().unwrap()[0].clone()));
}

fn main() {

    let duration = Duration::new(0, 1_000_000_000 / 60);

    println!("duration: {:?}", duration);

    let window = Window::new_centered("Speedy2D", (2560, 1600)).unwrap();

    //create data folder if it doesn't exist
    std::fs::create_dir_all("data").unwrap();

    //load nodes from nodes.data
    let mut file = OpenOptions::new().read(true).write(true).create(true).open("data/nodes.data").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let nodes_owned: Vec<Node> = if data.len() > 0 {
        serde_json::from_str(&data).unwrap()
    } else {
        Vec::new()
    };

    //get id as the maximum id of the nodes
    nodes_owned.iter().for_each(|node| {
        if node.get_id() > NODE_COUNTER.load(std::sync::atomic::Ordering::Relaxed) {
            NODE_COUNTER.store(node.get_id(), std::sync::atomic::Ordering::Relaxed);
        }
    });

    //refcellize the nodes vector
    nodes_owned.iter().for_each(|node| NODES.write().unwrap().push(Arc::new(RwLock::new(node.clone()))) );

    println!("duration: {:?}", duration);

    window.run_loop(RMaps {
        mouse_position: Vector2 { x: 0.0, y: 0.0 },
        side_panel: SidePanel::new(),
        top_panel: TopPanel::new(),
        last_left_click: SystemTime::now().sub(Duration::from_secs(1)),
        click_count: 0,
    });
}

struct RMaps {
    mouse_position : Vector2<f32>,
    side_panel: SidePanel,
    top_panel: TopPanel,

    last_left_click: SystemTime,
    click_count: i32,
}

impl Drop for RMaps {
    fn drop(&mut self) {

        for module in MODULES.read().unwrap().iter() {

            println!("unloading module \"{}\"", module.read().unwrap().get_name());
            module.write().unwrap().unload();

        }

        //save nodes to data/nodes.data using serde

        let mut file = File::create("data/nodes.data").unwrap();

        let mut nodes = Vec::new();
        for node in NODES.read().unwrap().iter() {
            nodes.push(node.read().unwrap().to_owned());
        }

        let serialized = serde_json::to_string(&nodes).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();

    }
}

impl WindowHandler for RMaps {
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        helper.request_redraw();
    }
    
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {

        let window_size = (helper.get_size_pixels().x as f32, helper.get_size_pixels().y as f32);

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        let top_panel_height = window_size.1 * top_panel::DEFAULT_HEIGHT_RATIO;
        let side_panel_width = window_size.0 * side_panel::DEFAULT_WIDTH_RATIO;

        self.side_panel.draw((side_panel_width, window_size.1-top_panel_height), (0.0,top_panel_height), helper, graphics);
        self.top_panel.draw((window_size.0, top_panel_height), (0.0,0.0), graphics);

        let size = Vec2::new(window_size.0-side_panel_width, window_size.1-top_panel_height);
        let offset = Vec2::new(side_panel_width, top_panel_height);

        ACTIVE_MODULE.read().unwrap().write().unwrap().draw(graphics, Rectangle::new(offset, offset+size));

        helper.request_redraw();

    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {

        if self.last_left_click.elapsed().unwrap().as_millis() < 400 {
            self.click_count += 1;
        }
        else {
            self.click_count = 1;
        }

        //check if mouse is contained in the side panel
        if self.side_panel.get_bounds().contains(self.mouse_position){
            self.side_panel.handle_click(self.mouse_position, self.click_count, button);
        }
        else if self.top_panel.get_bounds().contains(self.mouse_position){
            self.top_panel.handle_click(self.mouse_position, self.click_count, button);
        }
        else {
            ACTIVE_MODULE.read().unwrap().write().unwrap().handle_click(MousePosition::new(
                self.mouse_position,
                self.mouse_position - Vector2 { x: self.side_panel.get_bounds().width(), y: self.top_panel.get_bounds().height() },
            ), self.click_count);
        }

        self.last_left_click = SystemTime::now();

    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_position = position;
        self.last_left_click = SystemTime::now().sub(Duration::from_secs(1));
    }

    fn on_mouse_wheel_scroll(&mut self, helper: &mut WindowHelper<()>, distance: MouseScrollDistance) {
        ACTIVE_MODULE.read().unwrap().write().unwrap().handle_drag(MousePosition::new(
            self.mouse_position,
            self.mouse_position - Vector2 { x: self.side_panel.get_bounds().width(), y: self.top_panel.get_bounds().height() },
        ), distance);
    }

}
