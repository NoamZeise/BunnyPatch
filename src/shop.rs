use std::{path::Path, collections::HashMap};

use sdl_helper::{GameObject, Render, Error, Camera, input::Controls, geometry::Vec2};
use crate::tiles::Tiles;

pub struct Shop {
    bg: GameObject,
    shopkeep: GameObject,
    tile_btns: HashMap<Tiles, (GameObject, GameObject)>,
}

impl Shop {
    pub fn new(render: &mut Render) -> Result<Shop, Error> {
        let mut tile_btns = HashMap::new();
        tile_btns.insert(Tiles::Goat, 
                         (GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/goat.png"))?
                                 ),
                          GameObject::new_from_tex(
                              render.texture_manager.load(
                                  Path::new("resources/textures/btn/goat_select.png"))?)
                         )
        );

        let mut bg = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/shop.png")
            )?);
        bg.parallax = Vec2::new(0.0, 0.0);

        let mut shopkeep = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/shopkeep.png")
            )?
        );
        shopkeep.parallax = Vec2::new(0.0, 0.0);
        Ok(Shop {
            bg,
            shopkeep,
            tile_btns,
        })
    }

    pub fn update(&mut self, input: &Controls) {
        
    }

    pub fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.bg);
        cam.draw(&self.shopkeep);
    }
}
