use crate::structs::module::Module;
use crate::{ACTIVE_MODULE, MODULES};
use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::font::{Font, TextAlignment, TextLayout, TextOptions};
use speedy2d::shape::{Rect, Rectangle};
use speedy2d::window::MouseButton;
use speedy2d::window::WindowHelper;
use speedy2d::Graphics2D;
use std::sync::{Arc, RwLock};

pub const DEFAULT_WIDTH_RATIO: f32 = 0.20; //fixed width ratio over screen size
const BORDER_LINES_WIDTH: f32 = 2.0;
const TEXT_H_PADDING_RATIO: f32 = 0.05;
const TEXT_V_PADDING_RATIO: f32 = 0.01;
const TEXT_V_SPACING_RATIO: f32 = 0.015;
const BACKGROUND_COLOR: Color = Color::from_rgb(116.0 / 255.0, 140.0 / 255.0, 171.0 / 255.0);
const BORDER_LINES_COLOR: Color = Color::BLACK;
const TEXT_COLOR: Color = Color::BLACK;

static FONT_SIZE: f32 = 60.0;

pub struct SidePanel {
    bounds: Rectangle,
    hitboxes: Vec<(Rectangle, Arc<RwLock<Box<dyn Module + Send + Sync>>>)>,
    font: Font,
}

impl SidePanel {
    pub fn new() -> SidePanel {
        SidePanel {
            bounds: Rectangle::from_tuples((0.0, 0.0), (0.0, 0.0)),
            hitboxes: Vec::new(),
            font: Font::new(include_bytes!("../../res/OpenSans-SemiBold.ttf")).unwrap(),
        }
    }

    pub fn draw(
        &mut self,
        size: (f32, f32),
        offset: (f32, f32),
        helper: &mut WindowHelper,
        graphics: &mut Graphics2D,
    ) {
        self.bounds = Rectangle::from_tuples(offset, (size.0 + offset.0, size.1 + offset.1));

        let background = Rectangle::from_tuples(offset, (size.0 + offset.0, size.1 + offset.1));
        graphics.draw_rectangle(&background, BACKGROUND_COLOR);

        //draw border lines
        graphics.draw_line(
            self.bounds.top_left(),
            self.bounds.bottom_left(),
            BORDER_LINES_WIDTH,
            BORDER_LINES_COLOR,
        );
        graphics.draw_line(
            self.bounds.top_right(),
            self.bounds.bottom_right(),
            BORDER_LINES_WIDTH,
            BORDER_LINES_COLOR,
        );
        graphics.draw_line(
            self.bounds.top_left(),
            self.bounds.top_right(),
            BORDER_LINES_WIDTH,
            BORDER_LINES_COLOR,
        );
        graphics.draw_line(
            self.bounds.bottom_left(),
            self.bounds.bottom_right(),
            BORDER_LINES_WIDTH,
            BORDER_LINES_COLOR,
        );

        //draw modules
        let mut y = self.bounds.height() * TEXT_V_PADDING_RATIO + self.bounds.top_left().y;

        self.hitboxes.clear();

        for module in MODULES.read().unwrap().iter() {
            let formatted_text = self.font.layout_text(
                module.read().unwrap().get_name().as_str(),
                FONT_SIZE,
                TextOptions::new().with_wrap_to_width(
                    background.width() * (1.0 - TEXT_H_PADDING_RATIO),
                    TextAlignment::Left,
                ),
            );

            let hitbox = Rectangle::from_tuples(
                (
                    self.bounds.top_left().x + background.width() * TEXT_H_PADDING_RATIO,
                    y,
                ),
                (
                    self.bounds.top_left().x
                        + background.width() * TEXT_H_PADDING_RATIO
                        + formatted_text.width(),
                    y + formatted_text.height(),
                ),
            );

            graphics.draw_text(
                (hitbox.top_left().x, hitbox.top_left().y),
                TEXT_COLOR,
                &formatted_text,
            );
            y += self.bounds.height() * TEXT_V_SPACING_RATIO + formatted_text.height();

            self.hitboxes.push((hitbox, module.clone()));
        }
    }

    pub fn handle_click(&mut self, position: Vec2, click_count: i32, button: MouseButton) {
        println!("click at {:?}", position);
        for (hitbox, module) in &self.hitboxes {
            if hitbox.contains(position) {
                //I love this "write/unwrap" repetition lmao xD
                //I wonder if there's a better way to do this but I love it
                ACTIVE_MODULE.write().unwrap().write().unwrap().close();
                module.write().unwrap().open();
                *ACTIVE_MODULE.write().unwrap() = module.clone();
            }
        }
    }

    pub fn get_bounds(&self) -> Rectangle {
        self.bounds.clone()
    }
}
