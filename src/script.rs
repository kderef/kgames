pub struct Engine {
    engine: rhai::Engine,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            engine: rhai::Engine::new(),
        }
    }
}
