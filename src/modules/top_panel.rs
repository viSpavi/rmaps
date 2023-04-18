use speedy2d::color::Color;
use speedy2d::dimen::Vec2;
use speedy2d::shape::Rectangle;
use speedy2d::window::MouseButton;
use speedy2d::Graphics2D;

const BACKGROUND_COLOR: Color = Color::from_rgb(116.0 / 255.0, 140.0 / 255.0, 171.0 / 255.0);

pub const DEFAULT_HEIGHT_RATIO: f32 = 0.05;

pub struct TopPanel {
    bounds: Rectangle,
}

impl TopPanel {
    pub fn new() -> TopPanel {
        TopPanel {
            bounds: Rectangle::from_tuples((0.0, 0.0), (0.0, 0.0)),
        }
    }

    pub fn draw(&mut self, size: (f32, f32), offset: (f32, f32), graphics: &mut Graphics2D) {
        self.bounds = Rectangle::from_tuples(offset, (size.0 + offset.0, size.1 + offset.1));

        //draw background rectangle
        graphics.draw_rectangle(self.bounds.clone(), BACKGROUND_COLOR);
    }

    pub fn get_bounds(&self) -> Rectangle {
        self.bounds.clone()
    }

    pub fn handle_click(&mut self, position: Vec2, click_count: i32, button: MouseButton) {
        println!("clicked top panel at {position:?}")
    }
}
