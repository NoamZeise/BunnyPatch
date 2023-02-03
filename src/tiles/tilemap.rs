use super::{Tiles, Tile, types};

use sdl_helper::map::tiled;

pub struct Tilemap {
    pub map: Vec<Tiles>,
    pub map_updates: Vec<bool>,
    pub w: usize,
    pub h: usize,
}

impl Tilemap {
    pub fn new() -> Tilemap {
        Tilemap { map: Vec::new(), w: 0, h: 0, map_updates: Vec::new()}
    }

    pub fn set_map(&mut self, map: &tiled::Map) {
        self.w = map.width as usize;
        self.h = map.height as usize;

        self.map.clear();
        self.map.resize(self.w * self.h, Tiles::None);
        self.map_updates.clear();
        self.map_updates.resize(self.w * self.h, false);
        for layer in map.layers.iter() {
            if match layer.props.booleans.get("GameState") {
                Some(v) => *v,
                None => false,
            } {
                for x in 0..layer.width {
                    for y in 0..layer.height {
                        let i = self.bi(x as usize, y as usize);
                        //println!("Tile ID: {}", layer.tiles[i]);
                        self.map[i] = match layer.tiles[i] {
                            3 => Tiles::None,
                            0 => Tiles::Grass,
                            1 => Tiles::Root,
                            2 => Tiles::Carrot,
                            _ => panic!("unrecognized tile"),
                        }
                    }
                }
            }
        }
    }

    pub fn set_tile_objs(&self, tile_objs: &mut Vec<Box<dyn Tile>>) {
        tile_objs.clear();
        for x in 0..self.w {
            for y in 0..self.h {
                tile_objs.push(
                    self.set_tile_obj(self.get(x, y), x, y)
                )
            }
        }
    }

    pub fn set_tile_obj(&self, tile: Tiles, x: usize, y: usize) -> Box<dyn Tile> {
        match tile {
            Tiles::Root => Box::new(types::Root::new(x, y)),
            _ => Box::new(types::Empty::new(x, y)),
        }
    }

    pub fn set(&mut self, x: i64, y: i64, t: Tiles) {
        if !self.in_range_i(x, y) {
            return;
        }
        let i = self.bi(x as usize, y as usize);
        if self.map[i] == Tiles::Grass {
            self.map[i] = t;
            self.map_updates[i] = true;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tiles {
        let i = self.bi(x, y);
        self.map[i]
    }

    pub fn index(&self, i: usize) -> (usize, usize) {
        let y = i / self.w;
        println!("i:{}, x:{}, y:{}", i, i - (y * self.w), y);
        (i - (y * self.w), y)
    }

    fn bi(&self, x: usize, y:usize) -> usize {
        if !self.in_range(x, y) {
            eprintln!("out of range on board");
            return 0;
        }
        y * self.w + x
    }

    fn in_range(&self, x: usize, y: usize) -> bool {
        x < self.w && y < self.h
    }

    fn in_range_i(&self, x: i64, y: i64) -> bool {
        x < self.w as i64 && y < self.h as i64 &&
            x >= 0 && y >= 0
    }
}
