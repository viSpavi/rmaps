mod modules;
mod structs;
mod types;
mod utils;

use crate::g_node_container::generic_node_container::GenericNodeContainer;
use crate::node_container::node_container::NodeContainer;
use crate::modules::side_panel::SidePanel;
use crate::modules::top_panel::TopPanel;
use crate::modules::*;
use crate::structs::link::Link;
use crate::structs::module::Module;
use crate::structs::mouse_position::MousePosition;
use crate::structs::node::{Node, NODE_COUNTER};
use crate::types::DoublePointerSafe;
use lazy_static::lazy_static;
use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2, Vector2};
use speedy2d::shape::{Rect, Rectangle};
use speedy2d::window::{KeyScancode, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper, WindowPosition, WindowSize};
use speedy2d::{Graphics2D, Window};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, Instant, SystemTime};
use speedy2d::window::WindowSize::PhysicalPixels;

lazy_static! {
    static ref NODES: Arc<RwLock<Vec<Arc<RwLock<Node>>>>> = Arc::new(RwLock::new(Vec::new()));
    static ref LINKS: Arc<RwLock<Vec<Arc<RwLock<Link>>>>> = Arc::new(RwLock::new(Vec::new()));
    static ref MODULES: Arc<RwLock<Vec<Arc<RwLock<Box<dyn Module+Send+Sync>>>>>> = {
        let mut modules: Vec<Arc<RwLock<Box<dyn Module+Send+Sync>>>> = vec![
            Arc::new(RwLock::new(Box::new(GenericNodeContainer::new()))),
            Arc::new(RwLock::new(Box::new(NodeContainer::new()))),
        ];

        modules.iter_mut().for_each(|module| {
            module.write().unwrap().load();
        });
        Arc::new(RwLock::new(modules))
    };
    //double pointer. The main one (first arc/rwlock) is the static one while we modify the second one by replacing it with pointers to other modules.
    static ref ACTIVE_MODULE: DoublePointerSafe<Box<dyn Module+Send+Sync>> = Arc::new(RwLock::new(MODULES.read().unwrap()[0].clone()));
}

static REFRESH_RATE: f64 = 165.0*2.0;

fn main() {

    let window = Window::new_with_options(
        "RMaps", WindowCreationOptions::new_windowed(WindowSize::PhysicalPixels(UVec2::new(1500, 1000)), Some(WindowPosition::Center)).with_vsync(false)).unwrap();

    //new_centered("Speedy2D", (2560, 1600)).unwrap();

    //create data folder if it doesn't exist
    std::fs::create_dir_all("data").unwrap();

    //load nodes from nodes.data
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data/nodes.data")
        .unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let nodes_owned: Vec<Node> = if !data.is_empty() {
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
    NODE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    //refcellize the nodes vector
    nodes_owned.iter().for_each(|node| {
        NODES
            .write()
            .unwrap()
            .push(Arc::new(RwLock::new(node.clone())))
    });

    window.run_loop(RMaps {
        mouse_position: Vector2 { x: 0.0, y: 0.0 },
        side_panel: SidePanel::new(),
        top_panel: TopPanel::new(),
        last_left_click_up: SystemTime::UNIX_EPOCH,
        last_left_click_down: SystemTime::UNIX_EPOCH,
        click_count_up: 0,
        click_count_down: 0,
        delta_time: 1.0/REFRESH_RATE,
    });
}

struct RMaps {
    mouse_position: Vector2<f32>,
    side_panel: SidePanel,
    top_panel: TopPanel,

    last_left_click_up: SystemTime,
    last_left_click_down: SystemTime,
    click_count_up: i32,
    click_count_down: i32,
    delta_time: f64,
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

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, _size_pixels: UVec2) {
        helper.request_redraw();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {

        //start fps counter
        let start = Instant::now();

        helper.set_icon_from_rgba_pixels(vec![255, 1, 1, 255], (1, 1)).unwrap();

        let window_size = (
            helper.get_size_pixels().x as f32,
            helper.get_size_pixels().y as f32,
        );

        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        let top_panel_height = window_size.1 * top_panel::DEFAULT_HEIGHT_RATIO;
        let side_panel_width = window_size.0 * side_panel::DEFAULT_WIDTH_RATIO;
        let offset = Vec2::new(side_panel_width, top_panel_height);
        let size = Vec2::new(
            window_size.0 - side_panel_width,
            window_size.1 - top_panel_height,
        );

        ACTIVE_MODULE
            .read()
            .unwrap()
            .write()
            .unwrap()
            .draw(graphics, Rectangle::new(offset, offset + size), self.delta_time);

        self.side_panel.draw(
            (side_panel_width, window_size.1 - top_panel_height),
            (0.0, top_panel_height),
            helper,
            graphics,
        );
        self.top_panel
            .draw((window_size.0, top_panel_height), (0.0, 0.0), graphics);

        //end fps counter and wait to sync for refresh rate
        let end = Instant::now();
        let elapsed = end.duration_since(start).as_millis() as f64;
        let wait_time = 1000.0 / REFRESH_RATE - elapsed;
        if wait_time > 0.0 {
            //std::thread::sleep(Duration::from_millis(wait_time as u64));
        }
        //print fps
        //println!("fps: {}", 1000.0 / (Instant::now().duration_since(start).as_millis() as f64));

        //save delta_time as seconds and print it formatted to 2 decimal places
        self.delta_time = (elapsed + wait_time)/1000.0;
        //println!("delta_time: {}s", delta_time);


        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {

        //println!("{:?}", position);

        self.mouse_position = position;
        self.last_left_click_up = SystemTime::UNIX_EPOCH;
        self.last_left_click_down = SystemTime::UNIX_EPOCH;

        ACTIVE_MODULE.write().unwrap().write().unwrap().handle_mouse_move(MousePosition::new(
            self.mouse_position,
            self.mouse_position
                - Vector2 {
                x: self.side_panel.get_bounds().width(),
                y: self.top_panel.get_bounds().height(),
            },
        ),);

    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {

        if self.last_left_click_down.elapsed().unwrap().as_millis() < 400 {
            self.click_count_down += 1;
        } else {
            self.click_count_down = 1;
        }

        //check if mouse is contained in the side panel
        if self.side_panel.get_bounds().contains(self.mouse_position) {
            self.side_panel
                .handle_click(self.mouse_position, self.click_count_down, button);
        } else if self.top_panel.get_bounds().contains(self.mouse_position) {
            self.top_panel
                .handle_click(self.mouse_position, self.click_count_down, button);
        } else {
            ACTIVE_MODULE.read().unwrap().write().unwrap().handle_mouse_down(
                MousePosition::new(
                    self.mouse_position,
                    self.mouse_position
                        - Vector2 {
                            x: self.side_panel.get_bounds().width(),
                            y: self.top_panel.get_bounds().height(),
                        },
                ),
                self.click_count_down,
                button,
            );
        }

        self.last_left_click_down = SystemTime::now();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, button: MouseButton) {
        if self.last_left_click_up.elapsed().unwrap().as_millis() < 400 {
            self.click_count_up += 1;
        } else {
            self.click_count_up = 1;
        }

        //check if mouse is contained in the side panel
        if self.side_panel.get_bounds().contains(self.mouse_position) {
            self.side_panel
                .handle_click(self.mouse_position, self.click_count_up, button);
        } else if self.top_panel.get_bounds().contains(self.mouse_position) {
            self.top_panel
                .handle_click(self.mouse_position, self.click_count_up, button);
        } else {
            ACTIVE_MODULE.read().unwrap().write().unwrap().handle_mouse_up(
                MousePosition::new(
                    self.mouse_position,
                    self.mouse_position
                        - Vector2 {
                        x: self.side_panel.get_bounds().width(),
                        y: self.top_panel.get_bounds().height(),
                    },
                ),
                self.click_count_up,
                button,
            );
        }

        self.last_left_click_up = SystemTime::now();
    }

    fn on_mouse_wheel_scroll(&mut self, _: &mut WindowHelper<()>, distance: MouseScrollDistance, ) {
        ACTIVE_MODULE.read().unwrap().write().unwrap().handle_drag(
            MousePosition::new(
                self.mouse_position,
                self.mouse_position
                    - Vector2 {
                        x: self.side_panel.get_bounds().width(),
                        y: self.top_panel.get_bounds().height(),
                    },
            ),
            distance,
        );
    }

    fn on_key_down(&mut self, _helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {

        ACTIVE_MODULE.read().unwrap().write().unwrap().handle_key_down(virtual_key_code, scancode);
    }

    fn on_key_up(&mut self, _helper: &mut WindowHelper<()>, virtual_key_code: Option<VirtualKeyCode>, _scancode: KeyScancode) {

        ACTIVE_MODULE.read().unwrap().write().unwrap().handle_key_up(virtual_key_code, _scancode);
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<()>, unicode_codepoint: char) {

        ACTIVE_MODULE.read().unwrap().write().unwrap().handle_char(unicode_codepoint);
    }

}
