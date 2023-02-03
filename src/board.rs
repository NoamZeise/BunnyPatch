use crate::tiles::{Tiles, Tilemap, Tile};

use sdl_helper::input::Controls;
use sdl_helper::{geometry::*, GameObject, map::Map, Error, Render, Camera};
use std::collections::HashMap;
use std::path::Path;

const TILE: Vec2 = Vec2::new(19.0, 19.0);

pub struct Board {
    tile_texs: HashMap<Tiles, GameObject>,
    map: Option<Map>,
    obj_map: Vec<Box<dyn Tile>>,
    board: Tilemap,
    pos: Vec2,
}

impl Board {
    pub fn new(render: &mut Render) -> Result<Board, Error> {
        let tiles = Self::load_tiles(render)?;
        Ok(
            Board {
                tile_texs: tiles,
                board : Tilemap::new(),
                pos: Vec2::new(0.0, 0.0),
                map: None,
                obj_map: Vec::new(),
            }
        )
    }

    pub fn load_map(&mut self, path: &Path, render: &mut Render) -> Result<(), Error> {
        let map = Map::new(path, &mut render.texture_manager,
                           Path::new("resources/fonts/"),
                           &mut render.font_manager
        )?;
        self.board.set_map(&map.tiled_map);
        self.board.set_tile_objs(&mut self.obj_map);
        self.map = Some(map);
        Ok(())
    }

    pub fn update(&mut self, input: &Controls) {
        if input.kbm.press(sdl_helper::input::keyboard::Key::N) {
            for t in self.obj_map.iter_mut() {
                t.update(&mut self.board);
            }

            for (i, u) in self.board.map_updates.iter().enumerate() {
                if *u {
                    let (x, y) = self.board.index(i);
                    self.obj_map[i] = self.board.set_tile_obj(self.board.get(x, y), x, y);
                }
            }
        }
    }

    pub fn draw(&self, cam: &mut Camera) {
        match &self.map {
            None => (),
            Some(m) => {
                for i in 0..m.layers.len() {
                    if !match m.tiled_map.layers[i].props.booleans.get("GameState") {
                        Some(b) => *b,
                        None => false,
                    } {
                        m.layers[i].draw(cam)
                    }
                }
            }
        }
        self.draw_map(cam);
    }

    fn draw_map(&self, cam: &mut Camera) {
        for x in 0..self.board.w {
            for y in 0..self.board.h {
                match self.get_tile(x, y) {
                    Some(t) => { cam.draw(&t); }
                    None => {}
                }
            }
        }
    }

    fn load_tiles(render: &mut Render) -> Result<HashMap<Tiles, GameObject>, Error> {
        let mut tiles : HashMap<Tiles, GameObject> = HashMap::new();
        let mut tile_map = render.texture_manager.load(Path::new(
            "resources/textures/tiles/game_tiles.png"
        ))?;
        
        Self::load_tile(&mut tiles,
                        render,
                        Tiles::Root,
                        Path::new("resources/textures/tiles/root.png"))?;
        Self::load_tile(&mut tiles,
                        render,
                        Tiles::Carrot,
                        Path::new("resources/textures/tiles/carrot.png"))?;
        Self::load_tile(&mut tiles,
                        render,
                        Tiles::Grass,
                        Path::new("resources/textures/tiles/grass.png"))?;
        Self::load_tile(&mut tiles,
                        render,
                        Tiles::None,
                        Path::new("resources/textures/error.png"))?;
        Ok(tiles)
    }

    fn load_tile(tiles: &mut HashMap<Tiles, GameObject>, render: &mut Render, tile: Tiles, path: &Path) -> Result<(), Error> {
        let mut go = GameObject::new_from_tex(
            render.texture_manager.load(path)?
        );
        go.rect.w = TILE.x;
        go.rect.h = TILE.y;
        
        tiles.insert(tile, go);
        Ok(())
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<GameObject> {
        let tile = self.board.get(x, y);
        if tile == Tiles::None || tile == Tiles::Grass { return None; }
        let mut go = self.tile_texs.get(&tile).unwrap().clone();
        go.rect.x = self.pos.x + x as f64 * (TILE.x - 1.0);
        go.rect.y = self.pos.y + y as f64 * (TILE.y - 1.0);
        
        Some(go)
    }
}

