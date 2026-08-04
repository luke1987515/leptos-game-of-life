#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    generation: u64,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let mut universe = Self {
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
            generation: 0,
        };
        universe.random_fill();
        universe
    }

    pub fn random_fill(&mut self) {
        self.cells = (0..self.width * self.height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        self.generation = 0;
    }

    pub fn clear(&mut self) {
        self.cells.fill(Cell::Dead);
        self.generation = 0;
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let r = (row + delta_row) % self.height;
                let c = (col + delta_col) % self.width;
                count += self.cells[self.get_index(r, c)] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next[idx] = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }

        self.cells = next;
        self.generation += 1;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = match self.cells[idx] {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        };
    }

    pub fn live_count(&self) -> usize {
        self.cells.iter().filter(|&&c| c == Cell::Alive).count()
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn cells(&self) -> &[Cell] { &self.cells }
}