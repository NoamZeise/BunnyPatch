use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

const GROWTH_SPEED: usize = 2;

pub struct Root {
    pub x: usize,
    pub y: usize,
    pub growth: usize,
    pub age: usize,
    pub max_age: usize,
    pub res: Vec<GameObject>,
    pub current: GameObject,
    pub removed: bool,
    pub frozen: bool,
}

impl Root {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        let max_age = res.len() - 1;
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x,
            y,
            current,
            res,
            age: 0,
            max_age,
            removed: false,
            growth: 0,
            frozen: false,
        }
    }

    fn spread(&mut self, map: &mut Tilemap) {
        map.set(self.tile(), self.x as i64 + 1, self.y as i64, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 + 1, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 - 1, self.tile());
        map.set(self.tile(), self.x as i64 - 1, self.y as i64, self.tile());
    }

    fn behaviour(&mut self, map: &mut Tilemap) {
        if self.frozen {
            self.current.colour.r = 100;
            self.current.colour.g = 100;
            self.current.colour.b = 255;
            return;
        }
        
        if self.age != self.max_age{
            self.age += 1;
                self.current = self.res[self.age];
            self.current.rect.x = self.x as f64 * TILE.x;
            self.current.rect.y = self.y as f64 * TILE.y;
        } else if !self.frozen {
            self.spread(map);
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
        if other == Tiles::Grass || other == Tiles::Bush {
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
