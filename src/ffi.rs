use std::{cell::OnceCell, fmt::Display, path::PathBuf};

use macroquad::prelude::*;

use KeyCode::*;

use crate::{
    engine::dirs,
    texture::{asset_store, asset_store_mut, AssetStore},
};

// Macros
/// Macro for adding getters/setters to exposed types.
/// Example:
/// ```rust
/// reg_type! {
///     self.engine => {
///         Vec2 as "Vec2" = x, y;
///         Vec3 as "Vec3" = x, y, z;
///     }
///}
/// ```
#[macro_export]
macro_rules! reg_type {
    (
        $engine: expr => {
            $(
                $name:ty as $exposed_name:literal $(=
                    $($field:ident),*)?;
            )*
        }
    ) => {
        $(
            $(
                $engine.register_type_with_name::<$name>($exposed_name);

                // Register get/set for each field or method
                $(
                    $engine.register_get_set(
                        stringify!($field),
                        |_self: &mut $name| _self.$field,
                        |_self: &mut $name, new| _self.$field = new
                    );
                )*

            )*
        )?
    };
    // Getters via methods
    (
        $engine: expr => {
            $(
                $name:ty = $($method:ident()),*;
            )*
        }
    ) => {
       $(
           $(
                $engine.register_get(
                    stringify!($field),
                    |_self: &mut $name| _self.$method(),
                );
           )*
        )*
    };
}

/*
reg_type! {
    self.engine => {
        Vec2 as "Vec2" = r, g, b, a

    }
}
*/

// Constants

pub const COLORS: [(&'static str, Color); 26] = [
    ("LIGHTGRAY", LIGHTGRAY),
    ("GRAY", GRAY),
    ("DARKGRAY", DARKGRAY),
    ("YELLOW", YELLOW),
    ("GOLD", GOLD),
    ("ORANGE", ORANGE),
    ("PINK", PINK),
    ("RED", RED),
    ("MAROON", MAROON),
    ("GREEN", GREEN),
    ("LIME", LIME),
    ("DARKGREEN", DARKGREEN),
    ("SKYBLUE", SKYBLUE),
    ("BLUE", BLUE),
    ("DARKBLUE", DARKBLUE),
    ("PURPLE", PURPLE),
    ("VIOLET", VIOLET),
    ("DARKPURPLE", DARKPURPLE),
    ("BEIGE", BEIGE),
    ("BROWN", BROWN),
    ("DARKBROWN", DARKBROWN),
    ("WHITE", WHITE),
    ("BLACK", BLACK),
    ("BLANK", BLANK),
    ("MAGENTA", MAGENTA),
    // Custom
    ("WIN98_BG", Color::new(0.0, 0.50196078, 0.50196078, 1.0)),
];

pub const KEYS: [(&'static str, KeyCode); 121] = [
    ("KEY_SPACE", Space),
    ("KEY_APOSTROPHE", Apostrophe),
    ("KEY_COMMA", Comma),
    ("KEY_MINUS", Minus),
    ("KEY_PERIOD", Period),
    ("KEY_SLASH", Slash),
    ("KEY_KEY0", Key0),
    ("KEY_KEY1", Key1),
    ("KEY_KEY2", Key2),
    ("KEY_KEY3", Key3),
    ("KEY_KEY4", Key4),
    ("KEY_KEY5", Key5),
    ("KEY_KEY6", Key6),
    ("KEY_KEY7", Key7),
    ("KEY_KEY8", Key8),
    ("KEY_KEY9", Key9),
    ("KEY_SEMICOLON", Semicolon),
    ("KEY_EQUAL", Equal),
    ("KEY_A", A),
    ("KEY_B", B),
    ("KEY_C", C),
    ("KEY_D", D),
    ("KEY_E", E),
    ("KEY_F", F),
    ("KEY_G", G),
    ("KEY_H", H),
    ("KEY_I", I),
    ("KEY_J", J),
    ("KEY_K", K),
    ("KEY_L", L),
    ("KEY_M", M),
    ("KEY_N", N),
    ("KEY_O", O),
    ("KEY_P", P),
    ("KEY_Q", Q),
    ("KEY_R", R),
    ("KEY_S", S),
    ("KEY_T", T),
    ("KEY_U", U),
    ("KEY_V", V),
    ("KEY_W", W),
    ("KEY_X", X),
    ("KEY_Y", Y),
    ("KEY_Z", Z),
    ("KEY_LEFTBRACKET", LeftBracket),
    ("KEY_BACKSLASH", Backslash),
    ("KEY_RIGHTBRACKET", RightBracket),
    ("KEY_GRAVEACCENT", GraveAccent),
    ("KEY_WORLD1", World1),
    ("KEY_WORLD2", World2),
    ("KEY_ESCAPE", Escape),
    ("KEY_ENTER", Enter),
    ("KEY_TAB", Tab),
    ("KEY_BACKSPACE", Backspace),
    ("KEY_INSERT", Insert),
    ("KEY_DELETE", Delete),
    ("KEY_RIGHT", Right),
    ("KEY_LEFT", Left),
    ("KEY_DOWN", Down),
    ("KEY_UP", Up),
    ("KEY_PAGEUP", PageUp),
    ("KEY_PAGEDOWN", PageDown),
    ("KEY_HOME", Home),
    ("KEY_END", End),
    ("KEY_CAPSLOCK", CapsLock),
    ("KEY_SCROLLLOCK", ScrollLock),
    ("KEY_NUMLOCK", NumLock),
    ("KEY_PRINTSCREEN", PrintScreen),
    ("KEY_PAUSE", Pause),
    ("KEY_F1", F1),
    ("KEY_F2", F2),
    ("KEY_F3", F3),
    ("KEY_F4", F4),
    ("KEY_F5", F5),
    ("KEY_F6", F6),
    ("KEY_F7", F7),
    ("KEY_F8", F8),
    ("KEY_F9", F9),
    ("KEY_F10", F10),
    ("KEY_F11", F11),
    ("KEY_F12", F12),
    ("KEY_F13", F13),
    ("KEY_F14", F14),
    ("KEY_F15", F15),
    ("KEY_F16", F16),
    ("KEY_F17", F17),
    ("KEY_F18", F18),
    ("KEY_F19", F19),
    ("KEY_F20", F20),
    ("KEY_F21", F21),
    ("KEY_F22", F22),
    ("KEY_F23", F23),
    ("KEY_F24", F24),
    ("KEY_F25", F25),
    ("KEY_KP0", Kp0),
    ("KEY_KP1", Kp1),
    ("KEY_KP2", Kp2),
    ("KEY_KP3", Kp3),
    ("KEY_KP4", Kp4),
    ("KEY_KP5", Kp5),
    ("KEY_KP6", Kp6),
    ("KEY_KP7", Kp7),
    ("KEY_KP8", Kp8),
    ("KEY_KP9", Kp9),
    ("KEY_KPDECIMAL", KpDecimal),
    ("KEY_KPDIVIDE", KpDivide),
    ("KEY_KPMULTIPLY", KpMultiply),
    ("KEY_KPSUBTRACT", KpSubtract),
    ("KEY_KPADD", KpAdd),
    ("KEY_KPENTER", KpEnter),
    ("KEY_KPEQUAL", KpEqual),
    ("KEY_LEFTSHIFT", LeftShift),
    ("KEY_LEFTCONTROL", LeftControl),
    ("KEY_LEFTALT", LeftAlt),
    ("KEY_LEFTSUPER", LeftSuper),
    ("KEY_RIGHTSHIFT", RightShift),
    ("KEY_RIGHTCONTROL", RightControl),
    ("KEY_RIGHTALT", RightAlt),
    ("KEY_RIGHTSUPER", RightSuper),
    ("KEY_MENU", Menu),
    ("KEY_UNKOWN", Unknown),
];

pub const MOUSE_BUTTONS: [(&'static str, MouseButton); 4] = [
    ("MOUSE_LEFT", MouseButton::Left),
    ("MOUSE_RIGHT", MouseButton::Right),
    ("MOUSE_MIDDLE", MouseButton::Middle),
    ("MOUSE_UNKNOWN", MouseButton::Unknown),
];

#[cfg(feature = "rhai-engine")]
mod scripting {
    pub use rhai::{EvalAltResult, ImmutableString};
    pub type Error = Box<EvalAltResult>;
    pub type Result<T> = std::result::Result<T, Error>;
}
#[cfg(feature = "lua-engine")]
mod scripting {
    pub type Error = mlua::Error;
    pub type Result<T> = std::result::Result<T, Error>;
}

use scripting::*;

fn external_error(e: impl ToString + Display) -> scripting::Error {
    #[cfg(feature = "rhai-engine")]
    return Box::new(EvalAltResult::from(e.to_string()));
    #[cfg(feature = "lua-engine")]
    return scripting::Error::external(anyhow::anyhow!("{e}"));
}

/// Sync version of load_texture compatible with rhai
pub fn load_texture_sync(path: &str) -> scripting::Result<&Texture2D> {
    let complete_path = dirs().assets.join(path);

    let path_str = complete_path
        .to_str()
        .ok_or(format!(
            "Failed to convert path {path:?} to string (Maybe invalid UTF-8?)"
        ))
        .map_err(external_error)?;

    let texture = futures::executor::block_on(load_texture(path_str)).map_err(external_error)?;

    asset_store_mut()
        .user_textures
        .insert(path_str.to_string().into_boxed_str(), texture);

    // NOTE: it is inserted, so we know it exists
    Ok(unsafe { asset_store_mut().user_textures.get(path).unwrap_unchecked() })
}

/// Get stored texture (from engine)
pub fn load_texture_stored(name: &str) -> scripting::Result<&Texture2D> {
    asset_store()
        .get_texture(name)
        .ok_or(external_error(format!("Texture not found: '{name}'")))
}

pub fn draw_texture_stored(name: &str, x: f32, y: f32, tint: Color) -> scripting::Result<()> {
    let tex = load_texture_stored(name)?;
    draw_texture(tex, x, y, tint);
    Ok(())
}
