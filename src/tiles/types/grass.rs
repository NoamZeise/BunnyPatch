use crate::tiles::Tiles;
use crate::tiles::Tile;

pub struct Grass {
    pub x: usize,
    pub y: usize,
}

impl Grass {
    pub fn new(x: usize, y: usize) -> Self {
        Grass {x, y}
    }
}

impl Tile for Grass {
    fn tile(&self) -> Tiles {
        Tiles::Grass
    }
    fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
