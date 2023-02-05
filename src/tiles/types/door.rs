use sdl_helper::{GameObject, Camera};

use crate::tiles::{TILE, Tiles};

use super::super::Tile;


pub struct Door {
    pub pos: (usize, usize),
    res: Vec<GameObject>,
    current: GameObject
}

impl Door {
    pub fn new(x: usize, y: usize, res: Vec<GameObject>) -> Door{
        let mut current = res[0];
        current.rect.x = x as f64 * TILE.x;
        current.rect.y = y as f64 * TILE.y;
        Door {
            pos: (x, y),
            current,
            res,
        }
    }
}

impl Tile for Door {
    fn tile(&self) -> Tiles {
        Tiles::Door
    }
    fn pos(&self) -> (usize, usize) {
        self.pos
    }
    
    fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.current);
    }
}
