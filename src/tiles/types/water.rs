use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

pub struct Water {
    pub x: usize,
    pub y: usize,
    _res: Vec<GameObject>,
    current: GameObject,
    frozen: bool,
}

impl Water {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x, y, _res: res, current, frozen: false,
        }
    }

    fn set_tile(&mut self, x: i64, y: i64, map: &mut Tilemap) {
        let t = map.get_or_none(x, y);
        if t == Tiles::Carrot {
            map.set(self.tile(), x, y, self.tile());
        }
    }
}


impl Tile for Water {
    fn tile(&self) -> Tiles {
        Tiles::Water
    }
    
    fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    fn update(&mut self, map: &mut Tilemap) {
        if self.frozen {
            self.current.colour.r = 100;
            self.current.colour.g = 100;
            self.current.colour.b = 255;
            return;
        }
        self.set_tile(self.x as i64, self.y as i64 + 1, map);
        self.set_tile(self.x as i64, self.y as i64 - 1, map);
        self.set_tile(self.x as i64 + 1, self.y as i64, map);
        self.set_tile(self.x as i64 - 1, self.y as i64, map);
        self.set_tile(self.x as i64 + 1, self.y as i64 + 1, map);
        self.set_tile(self.x as i64 - 1, self.y as i64 + 1, map);
        self.set_tile(self.x as i64 + 1, self.y as i64 - 1, map);
        self.set_tile(self.x as i64 - 1, self.y as i64 - 1, map);
    }

    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }

    fn interact(&mut self, other: Tiles) {
        if other == Tiles::Ice {
            self.frozen = true;
        }
    }
}

