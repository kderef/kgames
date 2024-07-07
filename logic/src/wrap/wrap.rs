//! Wrap around macroquad functions with an interface

use super::ctx::Context;

/// Pointer to `dyn Drawer`
pub static mut MACROQUAD_CTX: *const Box<dyn Context> = std::ptr::null();

static mut COUNTER: usize = 0;

#[inline]
pub fn screen() -> &'static Box<dyn Context> {

    unsafe {
        COUNTER += 1;
        dbg!(COUNTER);
        &*MACROQUAD_CTX as &'static Box<dyn Context>
    }
}

#[no_mangle]
pub extern "C" fn set_ctx(drawer: *const Box<dyn Context>) {
    unsafe {
        COUNTER += 1;
        dbg!(COUNTER);
        MACROQUAD_CTX = drawer;
    }
}

#[cfg(debug_assertions)]
pub mod prelude {
    use super::screen;
    use macroquad::camera::Camera;
    pub use macroquad::prelude::{
        Texture2D, Rect, Vec2,
        KeyCode, Color, Font,
        TextDimensions,
        Camera2D, DrawRectangleParams,
        MouseButton, RenderTarget,
        vec2,
    };
    pub use macroquad::color::*;

    pub fn texture_empty() -> Texture2D {
        screen().empty_texture()
    }
    pub fn draw_circle(x: f32, y: f32, r: f32, color: Color) {
        screen().draw_circle(x, y, r, color)
    }
    pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thick: f32, color: Color) {
        screen().draw_line(x1, y1, x2, y2, thick, color)
    }
    pub fn set_default_camera() {
        screen().set_default_camera();
    }
    pub fn set_camera(camera: &dyn Camera) {
        screen().set_camera(camera);
    }
    pub fn render_target(w: u32, h: u32) -> RenderTarget {
        screen().render_target(w, h)
    }
    pub fn draw_rectangle_ex(x: f32, y: f32, w: f32, h: f32, params: DrawRectangleParams) {
        screen().draw_rectangle_ex(x, y, w, h, params)
    }
    pub fn mouse_position() -> (f32, f32) {
        screen().mouse_position()
    }
    pub fn is_mouse_button_down(mb: MouseButton) -> bool {
        screen().is_mouse_button_down(mb)
    }
    pub fn is_mouse_button_pressed(mb: MouseButton) -> bool {
        screen().is_mouse_button_pressed(mb)
    }
    pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
        screen().draw_rectangle(x, y, w, h, color)
    }
    pub fn get_frame_time() -> f32 {
        screen().get_frame_time()
    }
    pub fn clear_background(color: Color) {
        screen().clear_background(color);
    }
    pub fn screen_width() -> f32 {
        screen().screen_width()
    }
    pub fn screen_height() -> f32 {
        screen().screen_height()
    }
    pub fn is_key_down(key: KeyCode) -> bool {
        screen().is_key_down(key)
    }
    pub fn is_key_pressed(key: KeyCode) -> bool {
        screen().is_key_pressed(key)
    }
    pub fn measure_text(text: &str, font: Option<&Font>, font_size: u16, font_scale: f32) -> TextDimensions {
        screen().measure_text(text, font, font_size, font_scale)
    }
    pub fn draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        screen().draw_text(text, x, y, font_size, color)
    }
}

#[cfg(not(debug_assertions))]
pub mod prelude {
    pub use macroquad::prelude::*;

    #[inline]
    pub fn texture_empty() -> Texture2D {
        Texture2D::empty()
    }
}