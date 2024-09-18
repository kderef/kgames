mod common;
mod engine;

#[cfg(feature = "rhai-engine")]
pub mod rhai_engine;

#[cfg(feature = "lua-engine")]
pub mod lua_engine;

pub mod engine_impl {
    #[cfg(feature = "lua-engine")]
    pub use super::lua_engine::Engine;
    #[cfg(feature = "rhai-engine")]
    pub use super::rhai_engine::Engine;
}

pub use common::*;
pub use engine::*;
