use std::path::Path;

use sdl_helper::{Render, input::Controls, Camera, Error, GameObject, resource::Texture, geometry::Vec2};

use crate::tiles::{Tilemap, TILE, Tiles};



pub struct Ui {
    money: usize,
    money_icon: GameObject,
    money_tex: Vec<GameObject>,
    tiles: Texture,
    changed_money: bool,
    tile_to_set: Tiles,
    current_cursor: GameObject,
    cursor: GameObject,
}

impl Ui {
    pub fn new(render: &mut Render) -> Result<Ui, Error> {
        let mut money = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/carrot.png")
            )?
        );
        let cursor = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/cursor.png")
            )?
        );
        money.rect.x = 420.0;
        money.rect.y = 0.0;
        money.parallax = Vec2::new(0.0, 0.0);
        Ok(Ui {
            money: 10,
            money_icon: money,
            money_tex: Vec::new(),
            tiles: render.texture_manager.load(
                Path::new("resources/textures/tiles/game_tiles.png")
            )?,
            changed_money: true,
            tile_to_set: Tiles::None,
            current_cursor: cursor,
            cursor,
        })
    }

    pub fn update(&mut self, controls: &Controls) {
        if self.tile_to_set != Tiles::None {
            self.current_cursor = Tilemap::get_tile(self.tiles, 0, match self.tile_to_set {
                Tiles::Goat => 4,
                Tiles::Water => 5,
                Tiles::Bush => 6,
                Tiles::Ice => 7,
                Tiles::Key => 8,
                _ => panic!("not able to buy that tile"),

            });
        } else {
            self.current_cursor = self.cursor;
        }
        self.current_cursor.rect.x = controls.kbm.mouse_pos().x;
        self.current_cursor.rect.y = controls.kbm.mouse_pos().y;
        if self.changed_money {
            self.changed_money = false;
            self.money_tex = self.get_nums(self.money, self.money_icon.rect.centre());
        }
    }

    pub fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.money_icon);
        for m in self.money_tex.iter() {
            cam.draw(m);
        }
        cam.draw(&self.current_cursor);
    }

    fn get_digit(&mut self, num: usize, pos: Vec2, place: f64) -> GameObject {
        let mut t = Tilemap::get_tile(self.tiles, num, 3);
        t.parallax = Vec2::new(0.0, 0.0);
        t.rect.x = pos.x - TILE.x * 0.5 * place;
        t.rect.y = pos.y;
        return t;
    }

    pub fn get_nums(&mut self, num: usize, pos: Vec2) -> Vec<GameObject> {
        let mut n = Vec::new();

        let unit = num % 10;
        n.push(self.get_digit(unit, pos, 0.0));

        let mut order = 10;
        let mut index = 1.0;
        while num >= order {
            let d = (num / order) % 10;
            n.push(self.get_digit(d, pos, index));
            order *= 10;
            index += 1.0;
        }

        n.reverse();
        n
    }

    pub fn get_money(&self) -> usize {
        self.money
    }

    pub fn set_money(&mut self, m: usize) {
        self.money = m;
        self.changed_money = true;
    }

    pub fn inc_money(&mut self) {
        self.money+=1;
        self.changed_money = true;
    }

    pub fn set_tile(&mut self, t: Tiles) {
        self.tile_to_set = t;
    }

    pub fn get_tile(&mut self) -> Tiles {
        self.tile_to_set
    }

    pub fn pop_tile(&mut self) -> Tiles {
        let t = self.tile_to_set;
        self.tile_to_set = Tiles::None;
        t
    }
    
}
