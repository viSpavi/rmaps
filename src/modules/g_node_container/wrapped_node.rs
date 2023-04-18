use std::rc::Rc;
use std::sync::{Arc, RwLock};
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::shape::{Rect, RoundedRectangle, RoundRect};
use crate::modules::g_node_container::generic_node_container::{FONT_SIZE, ROUNDED_RECT_BORDER_RADIUS, ROUNDED_RECT_RADIUS, WRAPPED_NODE_BORDER_COLOR, WRAPPED_NODE_BORDER_SIZE, WRAPPED_NODE_COLOR, WRAPPED_NODE_PADDING, WRAPPED_NODE_SELECTED_COLOR};
use crate::structs::node::Node;
use serde::{Deserialize, Serialize};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::Graphics2D;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeWrapper {
    #[serde(skip)]
    node: Arc<RwLock<Node>>,
    node_id: i64,
    position: (f32, f32),
    #[serde(skip)]
    pub selected: bool,
    #[serde(skip, default = "default_cached_bounds")]
    cached_bounds: (Rect, f32),
    #[serde(skip, default = "default_vec2")]
    offset: Vec2,
}

fn default_cached_bounds() -> (Rect, f32) {
    (Rect::ZERO, 0.0)
}
fn default_vec2() -> Vec2 {
    Vec2::ZERO
}

impl NodeWrapper {

    pub fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.position
    }

    pub fn merge_offset(&mut self) {
        self.position.0 += self.offset.x;
        self.position.1 += self.offset.y;
        self.offset = Vec2::ZERO;
    }

    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    pub fn set_linked_node(&mut self, node: &Arc<RwLock<Node>>) {
        self.node = node.clone();
    }

    pub fn get_node(&self) -> Arc<RwLock<Node>> {
        self.node.clone()
    }

    pub fn get_node_id(&self) -> i64 {
        self.node_id
    }

    pub fn calculate_bounds(&mut self, from: &Rect, to: &Rect) -> RoundedRectangle {

        let (position, scale) = self.translation(from, to);

        let formatted_content = self.get_formatted_content(scale);
        let text_size = formatted_content.size();

        let outer_rect = RoundRect::from_tuples(
            (
                position.x - (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
                position.y - (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
            ),
            (
                position.x + text_size.x + (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
                position.y + text_size.y + (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
            ),
            ROUNDED_RECT_BORDER_RADIUS,
        );

        outer_rect
    }

    pub fn get_cached_bounds(&self) -> &(Rect, f32) {
        &self.cached_bounds
    }

    pub fn new_from_viewport(node: Arc<RwLock<Node>>, position: (f32, f32), from: &Rect, to: &Rect) -> NodeWrapper {
        let mut nw = NodeWrapper {
            node: Arc::clone(&node),
            node_id: node.read().unwrap().get_id(),
            position,
            selected: false,
            cached_bounds: default_cached_bounds(),
            offset: default_vec2(),
        };
        let (rect, _) = nw.translation(from, to);
        nw.set_position((rect.x, rect.y));
        nw
    }

    pub fn new(node: Arc<RwLock<Node>>, position: (f32, f32)) -> NodeWrapper {
        NodeWrapper {
            node: Arc::clone(&node),
            node_id: node.read().unwrap().get_id(),
            position,
            selected: false,
            cached_bounds: default_cached_bounds(),
            offset: default_vec2(),
        }
    }

    pub fn get_formatted_content(&self, scale: f32) -> Rc<FormattedTextBlock> {
        let font = Font::new(include_bytes!("../../../res/OpenSans-SemiBold.ttf")).unwrap();
        let formatted_content = font.layout_text(
            self.node.read().unwrap().get_content().as_str(),
            FONT_SIZE*scale,
            TextOptions::new(),
        );
        formatted_content
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D, from: &Rect, to: &Rect) {

        let (position, scale) = self.translation(from, to);
        let formatted_content = self.get_formatted_content(scale);
        let text_size = formatted_content.size();

        let outer_rect = RoundRect::from_tuples(
            (
                position.x - (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
                position.y - (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
            ),
            (
                position.x + text_size.x + (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
                position.y + text_size.y + (WRAPPED_NODE_BORDER_SIZE + WRAPPED_NODE_PADDING)*scale,
            ),
            ROUNDED_RECT_BORDER_RADIUS*scale,
        );
        self.cached_bounds = (Rect::new(position, position+formatted_content.size()), scale);


        graphics.draw_rounded_rectangle(outer_rect, *WRAPPED_NODE_BORDER_COLOR);

        let inner_rect = RoundRect::from_tuples(
            (
                position.x - WRAPPED_NODE_PADDING*scale,
                position.y - WRAPPED_NODE_PADDING*scale,
            ),
            (
                position.x + text_size.x + WRAPPED_NODE_PADDING*scale,
                position.y + text_size.y + WRAPPED_NODE_PADDING*scale,
            ),
            ROUNDED_RECT_RADIUS*scale,
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

    //first vector: position, second: scale, but only for the height. Screw the width
    pub fn translation(&self, from: &Rect, to: &Rect) -> (Vec2, f32) {
        let x = self.position.0 - from.top_left().x;
        let y = self.position.1 - from.top_left().y;

        let dx = x / from.width();
        let dy = y / from.height();

        let new_x = dx * to.width() + to.top_left().x;
        let new_y = dy * to.height() + to.top_left().y;

        let scale_y = to.height() / from.height();

        (Vec2::new(new_x, new_y)+self.offset, scale_y)
    }

}