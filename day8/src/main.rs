use std::fs;

struct Coord {
    x: usize,
    y: usize,
}

struct Tree {
    x: Option<usize>,
    y: Option<usize>,
    value: u32,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            x: None,
            y: None,
            value: 0,
        }
    }

    pub fn is_edge(&self) -> bool {
        self.x == Some(0) || self.y == Some(0) || self.x == Some(99) || self.y == Some(99)
    }

    pub fn get_neighbors_coordinates(&self) -> Vec<Coord> {
        match self.is_edge() {
            true => vec![],
            false => {}
        }
    }
}

fn main() {}
