use rand::{prelude::ThreadRng, Rng};
use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};



pub struct Root {
    pub x: usize,
    pub y: usize,
    pub age: usize,
    pub max_age: usize,
    pub res: Vec<GameObject>,
    pub current: GameObject,
    pub rng: ThreadRng,
    pub removed: bool,
}

impl Root {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        let max_age = res.len();
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x,
            y,
            current,
            res,
            age: 0,
            max_age,
            rng: rand::thread_rng(),
            removed: false,
        }
    }

    fn spread(&mut self, map: &mut Tilemap) {
        map.set(self.tile(), self.x as i64 + 1, self.y as i64, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 + 1, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 - 1, self.tile());
        map.set(self.tile(), self.x as i64 - 1, self.y as i64, self.tile());
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
        if self.age == self.max_age - 1 {
            self.spread(map);
        }
        self.age += 1;
        if self.age >= self.max_age {
            self.age = self.max_age - 1;
        } else {
            self.current = self.res[self.age];
            self.current.rect.x = self.x as f64 * TILE.x;
            self.current.rect.y = self.y as f64 * TILE.y;
        }
    }

    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }

    fn interact(&mut self, other: Tiles) {
        if other == Tiles::Grass {
            self.removed = true;
        }
    }

    fn removed(&mut self) -> bool {
        self.removed
    }
}
