use std::ops::{Index, IndexMut};

use macroquad::rand;

use super::cell::Cell;

const SCALE: usize = 14;

pub struct Grid {
    cells: Vec<Cell>,
    scratch: Vec<Cell>,
    width: usize,
    height: usize,
    size: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            cells: vec![Cell(false); size],
            scratch: vec![Cell(false); size],
            width,
            height,
            size,
        }
    }
    pub fn fill(&mut self, with: Cell) {
        self.cells.fill(with);
    }
    pub fn invert(&mut self) {
        for cell in &mut self.cells {
            cell.0 = !cell.0;
        }
    }
    pub fn fill_random(&mut self) {
        // NOTE: optimize maybe using bitshifting?
        for cell in &mut self.cells {
            cell.0 = (rand::rand() & 1) == 0;
        }
    }
    pub fn resize(&mut self, new_w: usize, new_h: usize) {
        if new_w <= self.width && new_h <= self.height {
            return;
        }

        let to_width = (new_w + SCALE - 1) / SCALE * SCALE;
        let to_height = (new_h + SCALE - 1) / SCALE * SCALE;
        let to_size = to_width * to_height;

        self.scratch.resize(to_size, Cell(false));
        self.cells.resize(to_size, Cell(false));

        self.scratch.copy_from_slice(&self.cells);

        // Move cells
        for y in 0..to_height.min(self.height) {
            for x in 0..to_width.min(self.width) {
                self.cells[y * self.width + x] = self.scratch[y * to_width + x];
            }
        }

        self.width = to_width;
        self.height = to_height;
        self.size = to_size;
    }
    pub fn tick(&mut self) {
        self.scratch.copy_from_slice(&self.cells);

        for y in 0..self.height {
            for x in 0..self.width {
                let state = self[y * self.width + x];
                let mut nbors = 0;

                for dx in -1..1 {
                    for dy in -1..1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if (0..self.width).contains(&(nx as usize))
                            && (0..self.height).contains(&(ny as usize))
                            && self[ny as usize * self.width + nx as usize].0
                        {
                            nbors += 1;
                        }
                    }
                }
                self.scratch[y * self.width + x] = state.next(nbors);
            }
        }
        self.cells.copy_from_slice(&self.scratch);
    }
}

impl Index<usize> for Grid {
    type Output = Cell;
    fn index(&self, index: usize) -> &Self::Output {
        self.cells.index(index)
    }
}
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.cells.index_mut(index)
    }
}
