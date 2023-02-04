use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

pub struct Goat {
    pub x: usize,
    pub y: usize,
    res: Vec<GameObject>,
    current: GameObject,
    charges: usize,
    used_charge: bool,
}

impl Goat {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x, y, res, current, charges: 4, used_charge: false,
        }
    }

    fn set_tile(&mut self, x: i64, y: i64, map: &mut Tilemap) {
        let t = map.get_or_none(x, y);
        if t == Tiles::Root || t == Tiles::Carrot {
            map.set(self.tile(), x, y, Tiles::Grass);
            self.used_charge = true;
        }
    }

    fn set_current(&mut self) {
        if self.charges == 0 { return; }
        self.current = self.res[4 - self.charges];
        self.current.rect.x = self.x as f64 * TILE.x;
        self.current.rect.y = self.y as f64 * TILE.y;
    }
}


impl Tile for Goat {
    fn tile(&self) -> Tiles {
        Tiles::Goat
    }
    
    fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    fn update(&mut self, map: &mut Tilemap) {
        self.used_charge = false;
        self.set_tile(self.x as i64, self.y as i64 + 1, map);
        self.set_tile(self.x as i64, self.y as i64 - 1, map);
        self.set_tile(self.x as i64 + 1, self.y as i64, map);
        self.set_tile(self.x as i64 - 1, self.y as i64, map);
        self.set_tile(self.x as i64 + 1, self.y as i64 + 1, map);
        self.set_tile(self.x as i64 - 1, self.y as i64 + 1, map);
        self.set_tile(self.x as i64 + 1, self.y as i64 - 1, map);
        self.set_tile(self.x as i64 - 1, self.y as i64 - 1, map);
        if self.charges > 0 && self.used_charge {
            self.charges -= 1;
            self.set_current();
        }
    }

    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }

    fn removed(&mut self) -> bool {
        self.charges == 0
    }
}
