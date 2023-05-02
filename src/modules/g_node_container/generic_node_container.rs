use std::cmp::min;
use crate::modules::g_node_container::generic_node_editor::GenericNodeEditor;
use crate::modules::g_node_container::key_bindings::*;
use crate::structs::module::Module;
use crate::structs::mouse_position::MousePosition;
use crate::structs::node::Node;
use crate::NODES;
use lazy_static::lazy_static;
use speedy2d::color::Color;
use speedy2d::shape::{Rect, Rectangle};
use speedy2d::window::{KeyScancode, MouseButton, MouseScrollDistance, VirtualKeyCode};
use speedy2d::Graphics2D;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::sync::{Arc, RwLock};
use speedy2d::dimen::Vec2;
use crate::modules::g_node_container::wrapped_node::NodeWrapper;

lazy_static! {
    pub static ref WRAPPED_NODE_BORDER_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);
    pub static ref WRAPPED_NODE_COLOR: Color = Color::from_hex_rgb(0xE5E0FF);
    pub static ref WRAPPED_NODE_SELECTED_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);
    static ref BACKGROUND_COLOR: Color = Color::from_hex_rgb(0xcad2c5);
    static ref SELECTION_RECTANGLE_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);
    static ref SELECTION_RECTANGLE_BORDER_COLOR: Color = Color::from_hex_rgb(0x8EA7E9);
}

const ZOOM_SPEED: f64 = 1.0/30.0;
const LERP_SPEED: f64 = 100.0;

pub const WRAPPED_NODE_BORDER_SIZE: f32 = 6.0;
pub const WRAPPED_NODE_PADDING: f32 = 20.0;
pub const ROUNDED_RECT_RADIUS: f32 = 14.0;
pub const ROUNDED_RECT_BORDER_RADIUS: f32 = 20.0;

pub static FONT_SIZE: f32 = 60.0;

pub struct GenericNodeContainer {
    wrapped_nodes: Vec<Arc<RwLock<NodeWrapper>>>,
    viewport: Rect,
    original_viewport: Rect,
    pivot: Vec2,
    node_editor: Option<GenericNodeEditor>,

    //auxiliary stuff
    target_viewport: Rect,
    drag_vector: Option<(Vec2, Vec2)>, //(start, move_vector)
    are_we_moving_nodes: Option<(Vec2, bool)>, //start, did_we_just_start_doing_that
    selection_rectangle: Option<(Vec2, Vec2)>, //(start, end)
}

impl GenericNodeContainer {
    pub fn new() -> GenericNodeContainer {
        GenericNodeContainer {
            wrapped_nodes: Vec::new(),
            viewport: Rect::from_tuples((0.0, 0.0), (0.0, 0.0)),
            original_viewport: Rect::from_tuples((0.0, 0.0), (0.0, 0.0)),
            target_viewport: Rect::from_tuples((0.0, 0.0), (0.0, 0.0)),
            pivot: Vec2::new(0.0, 0.0),
            node_editor: None,
            drag_vector: None,
            are_we_moving_nodes: None,
            selection_rectangle: None,
        }
    }

    pub fn get_selected_nodes(&self) -> Vec<Arc<RwLock<NodeWrapper>>> {
        self.wrapped_nodes.iter().filter(|wnode| wnode.read().unwrap().selected).map(|wnode| wnode.clone()).collect()
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
        }
        .into_iter()
        .map(|wnode| Arc::new(RwLock::new(wnode)))
        .collect();

        //link wrapped nodes to nodes. I hate that this is n^2 complexity.
        for node in NODES.read().unwrap().iter() {
            let id = node.read().unwrap().get_id();
            for wrapped_node in &mut self.wrapped_nodes {
                if wrapped_node.read().unwrap().get_node_id() == id {
                    wrapped_node.write().unwrap().set_linked_node(node);
                }
            }
        }
    }

    fn unload(&mut self) {
        //write wrapped nodes to a text
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("data/generic_node_container.data")
            .unwrap();
        let vector: Vec<NodeWrapper> = self
            .wrapped_nodes
            .iter_mut()
            .map(|wnode| wnode.read().unwrap().clone())
            .collect();
        serde_json::to_writer(file, &vector).unwrap();
    }

    fn get_name(&self) -> String {
        "Generic Node Container".to_string()
    }

    fn draw(&mut self, graphics: &mut Graphics2D, viewport: Rect, delta_time: f64) {

        self.original_viewport = viewport.clone();

        if self.target_viewport.height() < 0.1 || self.target_viewport.width() < 0.1 {
            self.target_viewport = viewport.clone();
        }

        let drag_vector = {
            if let Some((_, vector)) = self.drag_vector {
                vector
            } else {
                Vec2::new(0.0, 0.0)
            }
        };

        //lerp viewport
        self.viewport = Rectangle::from_tuples(
            (
                self.viewport.left() + (self.target_viewport.left() + drag_vector.x - self.viewport.left()) * ((LERP_SPEED*delta_time) as f32),
                self.viewport.top() + (self.target_viewport.top() + drag_vector.y - self.viewport.top()) * ((LERP_SPEED*delta_time) as f32),
            ),
            (
                self.viewport.right() + (self.target_viewport.right() + drag_vector.x - self.viewport.right()) * (LERP_SPEED*delta_time) as f32,
                self.viewport.bottom() + (self.target_viewport.bottom() + drag_vector.y - self.viewport.bottom()) * (LERP_SPEED*delta_time) as f32,
            ),
        );


        graphics.draw_rectangle(&viewport, *BACKGROUND_COLOR);

        //draw viewport
        graphics.draw_line(self.viewport.top_left(), self.viewport.top_right(), 2.0, Color::BLACK);
        graphics.draw_line(self.viewport.top_right(), self.viewport.bottom_right(), 2.0, Color::BLACK);
        graphics.draw_line(self.viewport.bottom_right(), self.viewport.bottom_left(), 2.0, Color::BLACK);
        graphics.draw_line(self.viewport.bottom_left(), self.viewport.top_left(), 2.0, Color::BLACK);

        graphics.draw_circle(self.pivot, 5.0, Color::BLUE);

        //draw selection rectangle
        if let Some((start, end)) = self.selection_rectangle {
            let rect = Rectangle::new(start, end);
            graphics.draw_rectangle(&rect, *SELECTION_RECTANGLE_COLOR);
        }

        //lerp nodes if we are dragging them around
        /*if self.are_we_moving_nodes.is_some() {
            self.wrapped_nodes.iter_mut().for_each(|wnode|
                wnode.write().unwrap().merge_offset_lerp(0.01)
            )
        }*/

        for wrapped_node in &self.wrapped_nodes {

            wrapped_node
                .write()
                .unwrap()
                .draw(graphics, &viewport, &self.viewport);
        }

        if let Some(node_editor) = &mut self.node_editor {
            node_editor.draw(graphics);
        }

    }

    fn close(&mut self) {}

    fn get_active_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        let mut map = HashMap::new();

        map.insert(vec![VirtualKeyCode::LControl, VirtualKeyCode::A], handle_enter());

        map
    }

    fn get_persistent_key_bindings(&self) -> HashMap<Vec<VirtualKeyCode>, Box<dyn Fn()>> {
        todo!()
    }

    fn handle_mouse_down(&mut self, mouse_position: MousePosition, click_count: i32, button: MouseButton) {

        let collisions: Vec<Arc<RwLock<NodeWrapper>>> = self.wrapped_nodes.iter_mut().filter(|wnode| {
            wnode.write().unwrap().calculate_bounds(&self.original_viewport, &self.viewport).contains(mouse_position.viewport())
        }).map(|wnode| wnode.clone()).collect();

        match button {
            MouseButton::Middle => {
                self.drag_vector = Some((mouse_position.viewport(), Vec2::new(0.0, 0.0)));
            }
            MouseButton::Left => {

                if collisions.is_empty() {
                    //start selection rectangle
                    self.selection_rectangle = Some((mouse_position.viewport(), mouse_position.viewport()));

                    self.node_editor = None;
                }

                else {
                    self.are_we_moving_nodes = Some((mouse_position.viewport(), true));
                }

                match click_count {
                    1 => {
                        for wrapped_node in collisions {
                            wrapped_node.write().unwrap().toggle_selected();
                        }
                    }
                    2 => {

                        if collisions.len() == 1 {
                            self.node_editor = Some(GenericNodeEditor::new(collisions[0].clone()));
                            collisions[0].write().unwrap().selected = true;
                        }

                        else {

                            let node = Node::create_and_register("new node".to_string(), self.get_name());

                            //retranslate position to the viewport
                            let x = (mouse_position.viewport().x - self.viewport.top_left().x) * self.original_viewport.width() / self.viewport.width() + self.original_viewport.top_left().x;
                            let y = (mouse_position.viewport().y - self.viewport.top_left().y) * self.original_viewport.height() / self.viewport.height() + self.original_viewport.top_left().y;

                            //self.pivot = Vec2::new(x, y);

                            let wrapped_node = NodeWrapper::new(Arc::new(RwLock::new(node)),
                                                                (x, y));

                            NODES.write().unwrap().push(wrapped_node.get_node());
                            self.wrapped_nodes.push(Arc::new(RwLock::new(wrapped_node)));
                        }
                    }
                    _ => {}
                }
            }
            MouseButton::Right => {

            }
            MouseButton::Other(_) => {}
        }


    }

    fn handle_mouse_up(&mut self, _position: MousePosition, _click_count: i32, button: MouseButton) {

        if button == MouseButton::Middle {
            //reset vector and make the translation permanent
            if let Some((_, vector)) = self.drag_vector.take() {
                self.target_viewport = Rectangle::new(
                    self.target_viewport.top_left() + vector,
                    self.target_viewport.bottom_right() + vector,
                );
            }
        }

        if button == MouseButton::Left {

            if let Some((start, end)) = self.selection_rectangle.take() {
                for wrapped_node in &mut self.wrapped_nodes {

                    let selection_rect = Rect::new(start, end);
                    let top = f32::min(selection_rect.top(), selection_rect.bottom());
                    let bottom = f32::max(selection_rect.top(), selection_rect.bottom());
                    let left = f32::min(selection_rect.left(), selection_rect.right());
                    let right = f32::max(selection_rect.left(), selection_rect.right());
                    let selection_rect = Rect::new(Vec2::new(left, top), Vec2::new(right, bottom));


                    let bounds = wrapped_node.write().unwrap().calculate_bounds(&self.original_viewport, &self.viewport);

                    //check if bounds are contained in selection rect
                    if selection_rect.contains(*bounds.top_left()) && selection_rect.contains(*bounds.bottom_right()) {
                        wrapped_node.write().unwrap().selected = true;
                    }
                }
            }

            for wnode in self.get_selected_nodes() {
                wnode.write().unwrap().merge_offset();
            }
            self.are_we_moving_nodes = None;
        }


    }

    fn handle_mouse_move(&mut self, position: MousePosition) {

        self.pivot = position.viewport();

        if let Some((_, end)) = &mut self.selection_rectangle {
            *end = position.viewport();
        }

        if let Some((start, didwe)) = self.are_we_moving_nodes {

            if didwe {

                //check collisions and select
                let collisions: Vec<Arc<RwLock<NodeWrapper>>> = self.wrapped_nodes.iter_mut().filter(|wnode| {
                    wnode.write().unwrap().calculate_bounds(&self.original_viewport, &self.viewport).contains(position.viewport())
                }).map(|wnode| wnode.clone()).collect();

                for wrapped_node in collisions {
                    wrapped_node.write().unwrap().selected = true;
                }

                self.are_we_moving_nodes = Some((start, false));
            }

            for wrapped_node in self.get_selected_nodes() {
                wrapped_node.write().unwrap().set_offset(position.viewport()-start);
            }
        }

        if let Some((start, vector)) = self.drag_vector {
            self.drag_vector = Some((start, position.viewport() - start));
        }
    }

    fn handle_drag(&mut self, position: MousePosition, distance: MouseScrollDistance) {
        let amount = match distance {
            MouseScrollDistance::Lines { x, y, z } => y * ZOOM_SPEED,
            _ => 0.0,
        } as f32;

        let a = self.target_viewport.top_left()-position.viewport();
        let b = self.target_viewport.bottom_right()-position.viewport();

        let rect = Rectangle::new(
            self.target_viewport.top_left()+a*amount,
            self.target_viewport.bottom_right()+b*amount,
        );
        self.target_viewport = rect;

    }

    fn handle_key_down(&mut self, _key: Option<VirtualKeyCode>, _scancode: KeyScancode) {
        if let Some(editor) = &mut self.node_editor {
            if let Some(key) = _key {
                editor.handle_key_down(key);
            }
        }
    }

    fn handle_key_up(&mut self, key: Option<VirtualKeyCode>, _scancode: KeyScancode) {

        if let Some(editor) = &mut self.node_editor {
            if let Some(key) = key {
                editor.handle_key_up(key);
            }
        }
    }

    fn handle_char(&mut self, character: char) {
        if let Some(editor) = &mut self.node_editor {
            editor.insert(character);
        }
        if character as u8 == 127 {
            self.wrapped_nodes.retain(|wnode| {
                if wnode.read().unwrap().selected {
                    let node = wnode.read().unwrap().get_node();
                    let id = node.read().unwrap().get_id();
                    NODES.write().unwrap().retain(|node| node.read().unwrap().get_id() != id);
                    false
                }
                else {
                    true
                }
            });
        }
    }

}
