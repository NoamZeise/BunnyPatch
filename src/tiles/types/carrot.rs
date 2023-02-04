use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

const MAX_HP: usize = 3;

pub struct Carrot {
    pub x: usize,
    pub y: usize,
    res: Vec<GameObject>,
    current: GameObject,
    hp: usize,
    no_dmg: bool,
}

impl Carrot {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[0];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x,
            y,
            current,
            res,
            hp: MAX_HP,
            no_dmg: true,
        }
    }

    fn set_current(&mut self) {
        self.current = self.res[MAX_HP - self.hp];
        self.current.rect.x = self.x as f64 * TILE.x;
        self.current.rect.y = self.y as f64 * TILE.y;
    }
}


impl Tile for Carrot {
    fn tile(&self) -> Tiles {
        Tiles::Carrot
    }
    
    fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    fn update(&mut self, _map: &mut Tilemap) {
        if self.no_dmg {
            self.hp += 1;
            if self.hp > MAX_HP {
                self.hp = MAX_HP;
            }
            self.set_current();
        }
        self.no_dmg = true;
    }

    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }

    fn removed(&mut self) -> bool {
        self.hp == 0
    }

    fn interact(&mut self, other: Tiles) {
        if other == Tiles::Root && self.hp != 0 {
            self.hp -= 1;
            self.no_dmg = false;
            if self.hp > 0 {
                self.set_current();
            }
        }
        if other == Tiles::Grass {
            self.hp = 0;
        }
    }
}
