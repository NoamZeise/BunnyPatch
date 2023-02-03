use std::{path::Path, collections::HashMap};

use sdl_helper::{Error, Render, GameObject, Camera, geometry::*, map::Map};

const TILE: Vec2 = Vec2::new(19.0, 19.0);

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum Tiles {
    None,
    Grass,
    Root,
    Carrot,
}

pub struct Board {
    tile_texs: HashMap<Tiles, GameObject>,
    map: Option<Map>,
    board_x: usize,
    board_y: usize,
    board: Vec<Tiles>,
    pos: Vec2,
}

impl Board {
    pub fn new(render: &mut Render) -> Result<Board, Error> {
        let tiles = Self::load_tiles(render)?;
        Ok(
            Board {
                tile_texs: tiles,
                board_x : 0,
                board_y : 0,
                board : Vec::new(),
                pos: Vec2::new(10.0, 10.0),
                map: None
            }
        )
    }

    pub fn load_map(&mut self, path: &Path, render: &mut Render) -> Result<(), Error> {
        let map = Map::new(path, &mut render.texture_manager,
                           Path::new("resources/fonts/"),
                           &mut render.font_manager
        )?;
        self.board_x = map.tiled_map.width as usize;
        self.board_y = map.tiled_map.height as usize;

        self.board.clear();
        self.board.resize(self.board_x * self.board_y, Tiles::None);
        for layer in map.tiled_map.layers.iter() {
            if match layer.props.booleans.get("GameState") {
                Some(v) => *v,
                None => false,
            } {
                for x in 0..layer.width {
                    for y in 0..layer.height {
                        let i = self.bi(x as usize, y as usize);
                        //println!("Tile ID: {}", layer.tiles[i]);
                        self.board[i] = match layer.tiles[i] {
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

        self.map = Some(map);
      
        Ok(())
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
        for x in 0..self.board_x {
            for y in 0..self.board_y {
                match self.get_tile(x, y) {
                    Some(t) => { cam.draw(&t); }
                    None => {}
                }
            }
        }
    }

    fn load_tiles(render: &mut Render) -> Result<HashMap<Tiles, GameObject>, Error> {
        let mut tiles : HashMap<Tiles, GameObject> = HashMap::new();
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
        let tile = self.board[self.bi(x, y)];
        if tile == Tiles::None || tile == Tiles::Grass { return None; }
        let mut go = self.tile_texs.get(&tile).unwrap().clone();
        go.rect.x = self.pos.x + x as f64 * (TILE.x - 1.0);
        go.rect.y = self.pos.y + y as f64 * (TILE.y - 1.0);
        
        Some(go)
    }

    fn set_tile(&mut self, x: usize, y: usize, tile: Tiles) {
        let i = self.bi(x, y);
        self.board[i] = tile;
    }

    fn bi(&self, x: usize, y:usize) -> usize {
        if x >= self.board_x || y >= self.board_y {
            panic!("out of range on board");
        }
        y * self.board_x + x
    }
}

