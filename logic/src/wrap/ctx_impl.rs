use super::ctx::Context;
use macroquad::prelude as mq;
use super::prelude::*;

pub struct DrawerImpl;
impl Context for DrawerImpl {
    fn empty_texture(&self) -> Texture2D {
        Texture2D::empty()
    }
    fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, color: Color) {
        mq::draw_line(x1, y1, x2, y2, thick, color)
    }
    fn set_default_camera(&self) {
        mq::set_default_camera();
    }
    fn set_camera(&self, camera: &dyn mq::Camera) {
        mq::set_camera(camera)
    }
    fn render_target(&self, w: u32, h: u32) -> RenderTarget {
        mq::render_target(w, h)
    }
    fn draw_rectangle_ex(&self, x: f32, y: f32, w: f32, h: f32, params: DrawRectangleParams) {
        mq::draw_rectangle_ex(x, y, w, h, params)
    }
    fn mouse_delta_position(&self) -> Vec2 {
        mq::mouse_delta_position()
    }
    fn mouse_position(&self) -> (f32, f32) {
        mq::mouse_position()
    }
    fn is_mouse_button_down(&self, mb: MouseButton) -> bool {
        mq::is_mouse_button_down(mb)
    }
    fn is_mouse_button_pressed(&self, mb: MouseButton) -> bool {
        mq::is_mouse_button_pressed(mb)
    }
    fn is_mouse_button_released(&self, mb: MouseButton) -> bool {
        mq::is_mouse_button_released(mb)
    }

    fn get_frame_time(&self) -> f32 {
        mq::get_frame_time()
    }
    fn is_key_down(&self, key: KeyCode) -> bool {
        mq::is_key_down(key)
    }
    fn is_key_pressed(&self, key: KeyCode) -> bool {
        mq::is_key_pressed(key)
    }
    fn is_key_released(&self, key: KeyCode) -> bool {
        mq::is_key_released(key)
    }
    fn measure_text(&self, text: &str, font: Option<&mq::Font>, font_size: u16, font_scale: f32) -> TextDimensions {
        mq::measure_text(text, font, font_size, font_scale)
    }
    fn screen_height(&self) -> f32 {
        mq::screen_height()
    }
    fn screen_width(&self) -> f32 {
        mq::screen_width()
    }
    fn clear_background(&self, color: Color) {
        mq::clear_background(color);
    }
    fn draw_rectangle(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        mq::draw_rectangle(x, y, w, h, color);
    }
    fn draw_circle(&self, x: f32, y: f32, radius: f32, color: Color) {
        mq::draw_circle(x, y, radius, color);
    }
    fn draw_text(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        mq::draw_text(text, x, y, font_size, color);
    }
}