use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Read;
use std::iter::Map;
use std::ops::Mul;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use lazy_static::lazy_static;
use speedy2d::color::Color;
use speedy2d::dimen::{Vec2, Vector2};
use speedy2d::Graphics2D;
use speedy2d::shape::{Rect, Rectangle, RoundedRectangle, RoundRect};
use speedy2d::window::{MouseScrollDistance, VirtualKeyCode};
use serde::{Deserialize, Serialize};
use speedy2d::font::{Font, TextLayout, TextOptions};
use crate::NODES;
use crate::structs::module::Module;
use crate::structs::mouse_position::MousePosition;
use crate::structs::node::Node;

lazy_static! {
    static ref WRAPPED_NODE_BORDER_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);
    static ref WRAPPED_NODE_COLOR: Color = Color::from_hex_rgb(0xE5E0FF);
    static ref WRAPPED_NODE_SELECTED_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);

}

const ZOOM_SPEED: f64 = 10.0;

const WRAPPED_NODE_BORDER_SIZE: f32 = 6.0;
const WRAPPED_NODE_PADDING: f32 = 20.0;
const ROUNDED_RECT_RADIUS: f32 = 14.0;
const ROUNDED_RECT_BORDER_RADIUS: f32 = 20.0;

static FONT_SIZE: f32 = 60.0;

pub struct GenericNodeContainer {
    wrapped_nodes: Vec<NodeWrapper>,
    viewport: Rect,
    pivot: Vec2,
}

impl GenericNodeContainer {

    pub fn new() -> GenericNodeContainer {
        GenericNodeContainer {
            wrapped_nodes: Vec::new(),
            viewport: Rect::from_tuples((0.0, 0.0), (0.0, 0.0)),
            pivot: Vec2::new(0.0,0.0),
        }
    }

}

//private, for now
#[derive(Clone, Debug, Serialize, Deserialize)]
struct NodeWrapper {
    #[serde(skip)]
    node: Arc<RwLock<Node>>,
    node_id: i64,
    position: (f32, f32),
    #[serde(skip)]
    selected: bool,
}

impl NodeWrapper {

    //todo: this is bad
    fn get_bounds(&self) -> RoundedRectangle {
        let font =  Font::new(include_bytes!("../../res/OpenSans-SemiBold.ttf")).unwrap();
        let formatted_content = font.layout_text(self.node.read().unwrap().get_content().as_str(), FONT_SIZE, TextOptions::new());
        let text_size = formatted_content.size();

        RoundedRectangle::from_tuples(
            (self.position.0 - WRAPPED_NODE_BORDER_SIZE - WRAPPED_NODE_PADDING, self.position.1 - WRAPPED_NODE_BORDER_SIZE - WRAPPED_NODE_PADDING),
            (self.position.0 + text_size.x + WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING, self.position.1 + text_size.y + WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING),
            ROUNDED_RECT_BORDER_RADIUS
        )
    }

    fn new(node: Arc<RwLock<Node>>, position: (f32, f32)) -> NodeWrapper {
        NodeWrapper {
            node:Arc::clone(&node),
            node_id: node.read().unwrap().get_id(),
            position,
            selected: false,
        }
    }

    fn draw(&self, graphics: &mut Graphics2D, from: &Rect, to: &Rect) {

        let position = self.translated_position(from, to);

        let font =  Font::new(include_bytes!("../../res/OpenSans-SemiBold.ttf")).unwrap();
        let formatted_content = font.layout_text(self.node.read().unwrap().get_content().as_str(), FONT_SIZE, TextOptions::new());
        let text_size = formatted_content.size();

        let outer_rect = RoundRect::from_tuples(
            (position.x - WRAPPED_NODE_BORDER_SIZE - WRAPPED_NODE_PADDING, position.y - WRAPPED_NODE_BORDER_SIZE - WRAPPED_NODE_PADDING),
            (position.x + text_size.x + WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING, position.y + text_size.y + WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING),
            ROUNDED_RECT_BORDER_RADIUS
        );

        graphics.draw_rounded_rectangle(outer_rect, *WRAPPED_NODE_BORDER_COLOR);

        let inner_rect = RoundRect::from_tuples(
            (position.x - WRAPPED_NODE_PADDING, position.y - WRAPPED_NODE_PADDING),
            (position.x + text_size.x + WRAPPED_NODE_PADDING, position.y + text_size.y + WRAPPED_NODE_PADDING),
            ROUNDED_RECT_RADIUS
        );

        graphics.draw_rounded_rectangle(inner_rect, {
            if self.selected {
                *WRAPPED_NODE_SELECTED_COLOR
            } else {
                *WRAPPED_NODE_COLOR
            }
        });

        //draw the contents
        graphics.draw_text(position, Color::BLACK, &formatted_content);

    }

    fn translated_position(&self, from: &Rect, to: &Rect) -> Vec2 {
        let x = self.position.0-from.top_left().x;
        let y = self.position.1-from.top_left().y;

        let dx = x / from.width();
        let dy = y / from.height();

        let new_x = dx * to.width() + to.top_left().x;
        let new_y = dy * to.height() + to.top_left().y;

        Vec2::new(new_x, new_y)
    }

}

impl Module for GenericNodeContainer {

    fn load(&mut self) {
        //read wrapped nodes from a text
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("data/generic_node_container.data")
            .unwrap();

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        self.wrapped_nodes = {
            if data.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&data).unwrap()
            }
        };

        //link wrapped nodes to nodes. I hate that this is n^2 complexity.
        for node in NODES.read().unwrap().iter() {
            let id = node.read().unwrap().get_id();
            for wrapped_node in &mut self.wrapped_nodes {
                if wrapped_node.node_id == id {
                    wrapped_node.node = Arc::clone(&node);
                }
            }
        }

    }

    fn unload(&mut self) {
        //write wrapped nodes to a text
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("data/generic_node_container.data")
            .unwrap();
        serde_json::to_writer(file, &self.wrapped_nodes).unwrap();
    }

    fn get_name(&self) -> String {
        "Generic Node Container".to_string()
    }

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect) {

        if self.viewport.height() < 0.1 || self.viewport.width() < 0.1 {
            self.viewport = viewport.clone();
        }

        graphics.draw_rectangle(&viewport, Color::WHITE);

        graphics.draw_circle(self.pivot, 5.0, Color::BLUE);

        for wrapped_node in &self.wrapped_nodes {
            wrapped_node.draw(graphics, &viewport, &self.viewport);
        }

    }

    fn open(&mut self) {
    }

    fn close(&mut self) {
    }

    fn get_active_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()> {
        todo!()
    }

    fn get_persistent_key_bindings(&self) -> Map<Vec<VirtualKeyCode>, fn()> {
        todo!()
    }

    fn handle_click(&mut self, mouse_position: MousePosition, click_count: i32) {

        let position = mouse_position.viewport();

        let mut selected = false;
        self.wrapped_nodes.iter_mut().for_each(|wnode| {
            if wnode.get_bounds().contains(position) {
                selected = true;
                wnode.selected = !wnode.selected;
            }
        });
        if selected { return }

        match click_count {
            1 => {

                self.pivot = mouse_position.viewport();

            }
            2 => {
                let node = Node::create_and_register("ayasdjfòlkajdsf".to_string(), "lmao".to_string());
                let id = node.get_id();

                let wrapped_node = NodeWrapper::new(
                    Arc::new(RwLock::new(node)),
                    (position.x, position.y),
                );

                NODES.write().unwrap().push(wrapped_node.node.clone());
                self.wrapped_nodes.push(wrapped_node);
            }
            _ => {}
        }
    }

    fn handle_drag(&mut self, position: MousePosition, distance: MouseScrollDistance) {


        let amount = match distance {
            MouseScrollDistance::Lines{x, y, z} => y*ZOOM_SPEED,
            _ => 0.0
        } as f32;

        println!("dragging by {} motherfuckers", amount);

    }

}