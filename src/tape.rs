#[derive(Debug)]
pub struct Tape {
    cells: Vec<u32>,
    head: usize,
}

impl Tape {
    pub fn new() -> Tape {
        Tape { cells: vec![0; 32], head: 0 }
    }

    fn resize(&mut self) {
        let len = self.cells.len();
        if self.head >= len {
            self.cells.resize(len * 2, 0);
        }
    }

    pub fn inc(&mut self) {
        self.cells[self.head] = self.cells[self.head].wrapping_add(1);
    }

    pub fn dec(&mut self) {
        self.cells[self.head] = self.cells[self.head].wrapping_sub(1);
    }

    pub fn fwd(&mut self) {
        self.head += 1;
        self.resize();
    }

    pub fn bwd(&mut self) {
        self.head -= 1;
        self.resize();
    }

    pub fn get(&self) -> u32 {
        self.cells[self.head]
    }

    pub fn put(&mut self, val: u32) {
        self.cells[self.head] = val;
    }
}
