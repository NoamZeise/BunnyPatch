use crate::tiles::Tilemap;

use super::super::{Tile, Tiles};

pub struct Root {
    pub x: usize,
    pub y: usize,
}

impl Root {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}


impl Tile for Root {
    fn tile(&self) -> Tiles {
        Tiles::Root
    }
    
    fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    fn update(&mut self, map: &mut Tilemap) {
        map.set(self.x as i64 + 1, self.y as i64, self.tile());
        map.set(self.x as i64 - 1, self.y as i64, self.tile());
        map.set(self.x as i64, self.y as i64 + 1, self.tile());
        map.set(self.x as i64, self.y as i64 - 1, self.tile());
        println!("root update");
    }
}
