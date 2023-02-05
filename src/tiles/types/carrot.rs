use sdl_helper::{GameObject, Camera};

use crate::tiles::Tilemap;

use super::super::{Tile, Tiles, tilemap::TILE};

const MAX_HP: usize = 3;
const HARVEST_TIME: usize = 4;
const WATER_SPREAD: usize = 6;

pub struct Carrot {
    pub x: usize,
    pub y: usize,
    res: Vec<GameObject>,
    current: GameObject,
    hp: usize,
    no_dmg: bool,
    harvestable: bool,
    no_dmg_turns: usize,
    turns_watered: usize,
    frozen: bool,
}

impl Carrot {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Self {
        let mut current = res[1];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Self {
            x,
            y,
            current,
            res,
            hp: MAX_HP,
            no_dmg: true,
            harvestable: false,
            no_dmg_turns: 2,
            turns_watered: 0,
            frozen: false,
        }
    }

    fn set_current(&mut self) {
        if self.harvestable {
            self.current = self.res[0];
        } else  {
            self.current = self.res[(MAX_HP + 1) - self.hp];
        }
        self.current.rect.x = self.x as f64 * TILE.x;
        self.current.rect.y = self.y as f64 * TILE.y;
    }

    fn spread(&mut self, map: &mut Tilemap) {
        map.set(self.tile(), self.x as i64 + 1, self.y as i64, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 + 1, self.tile());
        map.set(self.tile(), self.x as i64, self.y as i64 - 1, self.tile());
        map.set(self.tile(), self.x as i64 - 1, self.y as i64, self.tile());
    }
}


impl Tile for Carrot {
    fn tile(&self) -> Tiles {
        Tiles::Carrot
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
        if self.no_dmg {
            self.hp += 1;
            if self.hp > MAX_HP {
                self.hp = MAX_HP;
                if self.no_dmg_turns < HARVEST_TIME {
                    self.no_dmg_turns += 1;
                } else {
                    self.harvestable = true;
                    map.harvestable.push(self.pos());
                    self.no_dmg_turns = 0;
                }
            }
            self.set_current();
            self.harvestable = false;
            if self.turns_watered >= WATER_SPREAD {
                self.turns_watered = 0;
                self.spread(map);
            }
        } else {
            self.turns_watered = 0;
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
        if (other == Tiles::Root ) && self.hp != 0 {
            self.hp -= 1;
            self.no_dmg = false;
            if self.hp > 0 {
                self.set_current();
            }
        }
        if other == Tiles::Grass || other == Tiles::Bush {
            self.hp = 0;
        }
        if other == Tiles::Water {
            self.turns_watered += 1;
        }
        if other == Tiles::Ice {
            self.frozen = true;
        }
    }
}
