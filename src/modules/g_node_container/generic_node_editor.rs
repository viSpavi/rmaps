use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{VirtualKeyCode};
use speedy2d::Graphics2D;
use std::cmp::{max, min};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime};
use lazy_static::lazy_static;
use crate::modules::g_node_container::generic_node_container::{FONT_SIZE, WRAPPED_NODE_PADDING};
use crate::modules::g_node_container::wrapped_node::NodeWrapper;

lazy_static! {
    static ref EDITOR_COLOR: Color = Color::from_int_rgba(0, 0, 0, 255);
}

pub struct GenericNodeEditor {
    wrapped_node: Arc<RwLock<NodeWrapper>>,
    cursor_index: i32,
    selection: Option<(usize, usize)>,
    last_blink: SystemTime,
    selecting: bool,
}

impl GenericNodeEditor {
    pub fn new(node: Arc<RwLock<NodeWrapper>>) -> GenericNodeEditor {
        GenericNodeEditor {
            wrapped_node: node.clone(),
            cursor_index: node.read().unwrap().get_node().read().unwrap().get_content().chars().count() as i32,
            selection: None,
            selecting: false,
            last_blink: SystemTime::now(),
        }
    }

    pub fn move_cursor(&mut self, positions: i32) {
        self.cursor_index += positions;
        self.cursor_index = max(0, min(self.cursor_index, self.wrapped_node.read().unwrap().get_node().read().unwrap().get_content().chars().count() as i32));
    }

    pub fn insert(&mut self, character: char) {

        let mut characters = self.wrapped_node.read().unwrap().get_node().read().unwrap().get_content().clone().chars().collect::<Vec<char>>();

        if character == '\u{8}' {

            //if selecting, then remove selection
            if let Some(mut selection) = self.selection {
                let mut buffer = Vec::new();

                //normalize selection
                if selection.0 > selection.1 {
                    selection.1 = std::mem::replace(&mut selection.0, selection.1-1)-1;
                }

                for i in 0..characters.len() {
                    if i >= selection.0 && i <= selection.1 {
                        continue;
                    }
                    buffer.push(characters[i]);
                }

                self.wrapped_node.read().unwrap().get_node().write().unwrap().set_content(buffer.into_iter().collect::<String>());
                self.cursor_index = selection.0 as i32;
                self.selection = None;
                return;
            }

            if self.cursor_index == 0 {
                return;
            }

            let mut buffer = Vec::new();

            for i in 0..characters.len() {
                if i == self.cursor_index as usize - 1 {
                    continue;
                }
                buffer.push(characters[i]);
            }

            self.cursor_index -= 1;

            self.wrapped_node.read().unwrap().get_node().write().unwrap().set_content(buffer.into_iter().collect::<String>());
            return;

        }

        characters.insert(self.cursor_index as usize, character);

        self.wrapped_node.read().unwrap().get_node().write().unwrap().set_content(characters.into_iter().collect::<String>());

        self.cursor_index += 1;

    }

    pub fn handle_key_down(&mut self, key: VirtualKeyCode){

        //reset cursor timer
        self.last_blink = SystemTime::now();

        match key {
            //handle CTRL+A
            VirtualKeyCode::LControl | VirtualKeyCode::RControl => {

            }
            VirtualKeyCode::Left => {
                if let Some(mut selection) = self.selection {
                    selection.1  = self.cursor_index as usize;
                    self.selection = Some(selection);
                }
                if !self.selecting {
                    self.selection = None;
                }
                self.move_cursor(-1);
            },
            VirtualKeyCode::Right => {
                if let Some(mut selection) = self.selection {
                    selection.1 = self.cursor_index as usize;
                    self.selection = Some(selection);
                }
                if !self.selecting {
                    self.selection = None;
                }
                self.move_cursor(1);
            },
            VirtualKeyCode::LShift | VirtualKeyCode::RShift => {
                self.selecting = true;
                if self.selection.is_none() {
                    self.selection = Some((self.cursor_index as usize, self.cursor_index as usize));
                }
            },
            _ => {}

        }

    }

    pub fn handle_key_up(&mut self, key: VirtualKeyCode){

        match key {
            VirtualKeyCode::LShift | VirtualKeyCode::RShift => {
                self.selecting = false;
            },
            _ => {}
        }
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D) {

        //draw line on the bottom of the text
        let (rect, scale) = self.wrapped_node.read().unwrap().get_cached_bounds().clone();

        graphics.draw_line(rect.bottom_left()+Vec2::new(0.0, -WRAPPED_NODE_PADDING*scale*0.4),
                           rect.bottom_right()+Vec2::new(0.0, -WRAPPED_NODE_PADDING*scale*0.4), 5.0*scale,
                           Color::from_int_rgba(0, 0, 0, 255));

        //read formatted data to cursor position
        let font = Font::new(include_bytes!("../../../res/OpenSans-SemiBold.ttf")).unwrap();
        let formatted_content = font.layout_text(
            self.wrapped_node.read().unwrap().get_node().read().unwrap().get_content().chars().take(self.cursor_index as usize).collect::<String>().as_str(),
            FONT_SIZE*scale,
            TextOptions::new(),
        );

        //if selection is not none
        if let Some(selection) = self.selection {
            let formatted_content_b = font.layout_text(
                self.wrapped_node.read().unwrap().get_node().read().unwrap().get_content().chars().take(selection.0).collect::<String>().as_str(),
                FONT_SIZE*scale,
                TextOptions::new(),
            );
            //draw rectangle from selection
            graphics.draw_rectangle(
                Rectangle::new(
                    rect.top_left() + Vec2::new(formatted_content_b.width(), 0.0),
                    rect.top_left() + Vec2::new(formatted_content.width(), FONT_SIZE*scale),
                ),
                Color::from_int_rgba(0, 0, 255, 100),
            );
        }

        //if the time delta is odd, then draw the cursor
        if SystemTime::now().duration_since(self.last_blink).unwrap().as_secs() % 2 == 1 {
            return;
        }

        //draw line after the formatted content
        let a = rect.top_left() + Vec2::new(formatted_content.width(), 0.0);
        let b = rect.bottom_left() + Vec2::new(formatted_content.width(), 0.0);
        graphics.draw_line(a, b, 1.0, Color::from_int_rgba(0, 0, 0, 255));

    }
}
