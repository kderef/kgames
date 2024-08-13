#[derive(Clone, Copy)]
pub struct Cell(pub bool);

impl Cell {
    pub fn kill(&mut self) {
        self.0 = false;
    }

    pub fn alive(self) -> bool {
        self.0
    }
    pub fn dead(self, nbors: usize) -> bool {
        !self.alive()
    }

    pub fn next(self, nbors: usize) -> Self {
        Self(if self.alive() {
            nbors >= 2 && nbors <= 3
        } else {
            nbors == 3
        })
    }
}
