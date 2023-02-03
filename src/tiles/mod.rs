
mod tilemap;
pub mod types;
pub use tilemap::Tilemap;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum Tiles {
    None,
    Grass,
    Root,
    Carrot,
}

pub trait Tile {
    fn tile(&self) -> Tiles {
        Tiles::None
    }

    fn pos(&self) -> (usize, usize);

    fn removed(&mut self) -> bool {
        false
    }

    fn update(&mut self, _: &mut Tilemap) {
        
    }
}
