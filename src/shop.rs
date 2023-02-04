use std::{path::Path, collections::HashMap};

use sdl_helper::{GameObject, Render, Error, Camera, input::Controls, geometry::Vec2};
use crate::tiles::Tiles;
use crate::button::Button;
use crate::ui::Ui;

struct ShopBtn {
    pub btn: Button,
    pub cost: usize,
    pub t: Tiles,
    pub set: bool,
    pub price_num: Vec<GameObject>,
    pub pos: Vec2,
}

impl ShopBtn {
    pub fn new(t: Tiles, cost: usize, normal: GameObject, active: GameObject, pos: Vec2) -> Self {
        ShopBtn {
            btn: Button::new(normal, active, pos),
            cost,
            t,
            set: false,
            price_num: Vec::new(),
            pos,
        }
    }

    pub fn set(&mut self, ui: &mut Ui) {
        if self.set { return; }
        self.set = true;
        self.price_num = ui.get_nums(self.cost, self.pos + Vec2::new(40.0, 20.0));
    }
}

pub struct Shop {
    bg: GameObject,
    shopkeep: GameObject,
    tile_btns: Vec<ShopBtn>,
    exit: Button,
    closed: bool,
}

impl Shop {
    pub fn new(render: &mut Render) -> Result<Shop, Error> {
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
            tile_btns: vec![
                ShopBtn::new(Tiles::Goat, 5, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/goat.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/goat_active.png"))?),
                             Vec2::new(200.0, 100.0)
                ),
            ],
            exit: Button::new(
                GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/exit.png")
                )?
            ),
                GameObject::new_from_tex(
                    render.texture_manager.load(
                        Path::new("resources/textures/btn/exit_active.png")
                    )?
                ),
                Vec2::new(410.0, 40.0)
            ),
            closed: false,
        })
    }

    pub fn update(&mut self, input: &Controls, ui: &mut Ui) {
        for btn in self.tile_btns.iter_mut() {
            btn.btn.update(input);
            if !btn.set {
                btn.set(ui);
            }
            if btn.btn.clicked() {
                if ui.get_money() >= btn.cost {
                    ui.set_money(ui.get_money() - btn.cost);
                    ui.set_tile(btn.t);
                    self.closed = true;
                }
            }
        }
        self.exit.update(input);
    }

    pub fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.bg);
        cam.draw(&self.shopkeep);
        for btn in self.tile_btns.iter() {
            btn.btn.draw(cam);
            for n in btn.price_num.iter() {
                cam.draw(n);
            }
        }
        self.exit.draw(cam);
    }

    pub fn change(&mut self) -> bool {
        let c = self.closed;
        self.closed = false;
        self.exit.clicked() || c
    }
}
