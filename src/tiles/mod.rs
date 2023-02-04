
mod tilemap;
pub mod types;
use sdl_helper::Camera;
pub use tilemap::{Tilemap, Choice, TILE};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Tiles {
    None,
    Grass,
    Root,
    Carrot,
    Goat,
}

pub trait Tile {
    fn tile(&self) -> Tiles {
        Tiles::None
    }

    fn pos(&self) -> (usize, usize);

    fn removed(&mut self) -> bool {
        false
    }

    fn update(&mut self, _map: &mut Tilemap) {
        
    }

    fn draw(&self, _cam: &mut Camera) {

    }

    fn interact(&mut self, _tile:Tiles) {
        
    }
}
