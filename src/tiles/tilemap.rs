use std::collections::HashMap;

use super::{Tiles, Tile, types};

use sdl_helper::{map::tiled::{self, Text}, GameObject, resource::Texture, geometry::*, Colour};

pub const TILE: Vec2 = Vec2::new(19.0, 19.0);

#[derive(Clone, Copy)]
pub struct Choice {
    pub i: usize,
    pub x: usize,
    pub y: usize,
    pub src: Tiles,
    pub dst: Tiles,
}

pub struct Tilemap {
    pub map: Vec<Tiles>,
    pub map_updates: Vec<Choice>,
    pub w: usize,
    pub h: usize,
    pub resources: HashMap<Tiles, Vec<GameObject>>,
    pub harvestable: Vec<(usize, usize)>,
}

impl Tilemap {
    pub fn new(tiles: Texture) -> Tilemap {
        Tilemap {
            map: Vec::new(), w: 0, h: 0,
            map_updates: Vec::new(),
            resources: Self::load_resources(tiles),
            harvestable: Vec::new(),
        }
    }

    fn load_resources(tiles: Texture) -> HashMap<Tiles, Vec<GameObject>> {
        let mut r = HashMap::new();
        r.insert(Tiles::None, vec![
                     Self::get_tile(tiles, 0, 3)]);
        r.insert(Tiles::Grass, Vec::new());
        r.insert(Tiles::Root,
                 vec![
                     Self::get_tile(tiles, 0, 1),
                     Self::get_tile(tiles, 1, 1),
                     Self::get_tile(tiles, 2, 1),
                     Self::get_tile(tiles, 3, 1),
                 ]
        );
        r.insert(Tiles::Carrot,
                 vec![
                     Self::get_tile(tiles, 0, 0),
                     Self::get_tile(tiles, 1, 0),
                     Self::get_tile(tiles, 2, 0),
                 ]
        );
        r.insert(Tiles::Goat,
                 vec![
                     Self::get_tile(tiles, 0, 4),
                     Self::get_tile(tiles, 1, 4),
                     Self::get_tile(tiles, 2, 4),
                     Self::get_tile(tiles, 3, 4),
                     Self::get_tile(tiles, 4, 4),
                 ]
        );
        r
    }

    pub fn get_tile(tex: Texture, x: usize, y: usize) -> GameObject {
        GameObject::new(
            tex,
            Rect::new(0.0, 0.0, TILE.x, TILE.y),
            Rect::new(
                x as f64 * TILE.x, y as f64 * TILE.y, TILE.x, TILE.y
            ),
            Vec2::new(1.0, 1.0), Colour::white()
        )
    }

    pub fn set_map(&mut self, map: &tiled::Map) {
        self.w = map.width as usize;
        self.h = map.height as usize;

        self.map.clear();
        self.map.resize(self.w * self.h, Tiles::None);
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
        for y in 0..self.h {
            for x in 0..self.w {
                tile_objs.push(
                    self.set_tile_obj(self.get(x, y), x, y)
                )
            }
        }
    }

    pub fn set_tile_obj(&self, tile: Tiles, x: usize, y: usize) -> Box<dyn Tile> {
        match tile {
            Tiles::Root => Box::new(
                types::Root::new(x, y, self.resources.get(&Tiles::Root).unwrap().clone())
            ),
            Tiles::Grass => Box::new(
                types::Grass::new(x, y),
            ),
            Tiles::Carrot => Box::new(
                types::Carrot::new(x, y, self.resources.get(&Tiles::Carrot).unwrap().clone()),
            ),
            Tiles::Goat => {
                Box::new(
                types::Goat::new(x, y, self.resources.get(&Tiles::Goat).unwrap().clone()),
            )},
            _ => Box::new(types::Empty::new(x, y)),
        }
    }

    pub fn set(&mut self, src: Tiles, x: i64, y: i64, t: Tiles) {
        if !self.in_range_i(x, y) {
            return;
        }
        let i = self.bi(x as usize, y as usize);
        if self.map[i] == Tiles::None { return; }
        self.map_updates.push(
            Choice {
                i,
                x: x as usize, y: y as usize, src, dst: t
            }
        );
    }

    pub fn get(&self, x: usize, y: usize) -> Tiles {
        let i = self.bi(x, y);
        self.map[i]
    }

    pub fn get_or_none(&self, x: i64, y: i64) -> Tiles {
        if !self.in_range_i(x, y) {
            return Tiles::None;
        }
        let i = self.bi(x as usize, y as usize);
        self.map[i]
    }

    pub fn _index(&self, i: usize) -> (usize, usize) {
        let y = i / self.w;
        (i - (y * self.w), y)
    }

    pub fn bi(&self, x: usize, y:usize) -> usize {
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
