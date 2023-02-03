use super::super::Tile;


pub struct Empty {
    pub pos: (usize, usize),
}

impl Empty {
    pub fn new(x: usize, y: usize) -> Empty{
        Empty {
            pos: (x, y),
        }
    }
}

impl Tile for Empty {
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
}
