use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

const GROWTH_SPEED: usize = 8;

pub struct Bush {
    pub x: usize,
    pub y: usize,
    pub growth: usize,
    pub res: Vec<GameObject>,
    pub current: GameObject,
    pub removed: bool,
    pub frozen: bool,
}

impl Bush {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x,
            y,
            current,
            res,
            removed: false,
            growth: 0,
            frozen: false,
        }
    }

    fn spread(&mut self, map: &mut Tilemap) {
        map.set(self.tile(), self.x as i64 - 1, self.y as i64 - 1, self.tile());
        map.set(self.tile(), self.x as i64 + 1, self.y as i64 + 1, self.tile());
    }

    fn behaviour(&mut self, map: &mut Tilemap) {
        if self.frozen {
            self.current.colour.r = 100;
            self.current.colour.g = 100;
            self.current.colour.b = 255;
            return;
        }
        self.spread(map);
    }
}


impl Tile for Bush {
    fn tile(&self) -> Tiles {
        Tiles::Bush
    }
    
    fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    fn update(&mut self, map: &mut Tilemap) {
        self.growth += 1;
        if self.growth >= GROWTH_SPEED {
            self.growth = 0;
            self.behaviour(map);
        }
    }

    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }

    fn interact(&mut self, other: Tiles) {
        if other == Tiles::Grass {
            self.removed = true;
        }
        if other == Tiles::Ice {
            self.frozen = true;
        }
    }

    fn removed(&mut self) -> bool {
        self.removed
    }
}
