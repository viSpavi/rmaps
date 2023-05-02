use speedy2d::dimen::Vec2;
use speedy2d::shape::Rect;

//function that translates a point from window position to a viewport position, taking into account translation and scale
pub fn window_to_viewport(point: Vec2, window: Vec2, viewport: Rect) -> Vec2 {
    let x = (point.x - window.x/2.0)/window.x;
    let y = (point.y - window.y/2.0)/window.y;
    let x = x*viewport.width() + viewport.left();
    let y = y*viewport.height() + viewport.top();
    Vec2::new(x, y)
}

pub fn viewport_to_window(point: Vec2, window: Vec2, viewport: Rect) -> Vec2 {
    let x = (point.x - viewport.left())/viewport.width();
    let y = (point.y - viewport.top())/viewport.height();
    let x = x*window.x + window.x/2.0;
    let y = y*window.y + window.y/2.0;
    Vec2::new(x, y)
}