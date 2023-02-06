use std::{path::Path, collections::HashMap};

use sdl_helper::Colour;
use sdl_helper::geometry::Rect;
use sdl_helper::{GameObject, Render, Error, Camera, input::Controls, geometry::Vec2};
use crate::tiles::Tiles;
use crate::button::Button;
use crate::ui::Ui;

const OPEN_TIME : f64 = 0.8;
const CLOSE_TIME: f64 = 0.4;

struct ShopBtn {
    pub btn: Button,
    pub cost: usize,
    pub t: Tiles,
    pub set: bool,
    pub price_num: Vec<GameObject>,
    price_rects: Vec<Rect>,
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
            price_rects: Vec::new(),
            pos,
        }
    }

    pub fn set(&mut self, ui: &mut Ui) {
        if self.set { return; }
        self.set = true;
        self.price_num = ui.get_nums(self.cost, self.pos + Vec2::new(70.0, 20.0));
        self.price_rects.clear();
        for p in self.price_num.iter() {
            self.price_rects.push(p.rect);
        }
    }

    pub fn set_pos(&mut self, change: Vec2) {
        self.btn.set_pos(self.pos + change);
        for (i, n) in self.price_num.iter_mut().enumerate() {
            n.rect.x = self.price_rects[i].x + change.x;
            n.rect.y = self.price_rects[i].y + change.y;
        }
    }
}

pub struct Shop {
    bg: GameObject,
    fade: GameObject,
    shopkeep: GameObject,
    since_opened: f64,
    tile_btns: Vec<ShopBtn>,
    exit: Button,
    exit_og: Rect,
    close_time: f64,
    closed: bool,
    first: bool,
}

impl Shop {
    pub fn new(render: &mut Render) -> Result<Shop, Error> {
        let mut fade = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/tiles/blank.png"))?);
        fade.rect.w = 500.0;
        fade.rect.h = 500.0;
        fade.colour = Colour::new(10, 10, 10, 100);
        fade.parallax = Vec2::new(0.0, 0.0);
        let mut bg = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/shop.png")
            )?);
        bg.parallax = Vec2::new(0.0, 0.0);

        let exit = Button::new(
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
        );
        let exit_og = exit.get_rect();
        
        let mut shopkeep = GameObject::new_from_tex(
            render.texture_manager.load(
                Path::new("resources/textures/shopkeep.png")
            )?
        );
        
        shopkeep.parallax = Vec2::new(0.0, 0.0);
        let base_btn = Vec2::new(200.0, 100.0);
        Ok(Shop {
            since_opened: 0.0,
            bg,
            fade,
            first: true,
            shopkeep,
            tile_btns: vec![
                ShopBtn::new(Tiles::Goat, 20, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/goat.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/goat_active.png"))?),
                             base_btn,
                ),
                ShopBtn::new(Tiles::Bush, 40, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/bush.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/bush_active.png"))?),
                             base_btn + Vec2::new(0.0, 50.0)
                ),
                ShopBtn::new(Tiles::Water, 120, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/water.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/water_active.png"))?),
                             base_btn + Vec2::new(100.0, 0.0)
                ),
                ShopBtn::new(Tiles::Ice, 70, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/ice.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/ice_active.png"))?),
                             base_btn + Vec2::new(100.0, 50.0)
                ),
                ShopBtn::new(Tiles::Key, 1000, GameObject::new_from_tex(
                             render.texture_manager.load(
                                 Path::new("resources/textures/btn/key.png"))?),
                             GameObject::new_from_tex(
                                 render.texture_manager.load(
                                     Path::new("resources/textures/btn/key_active.png"))?),
                             base_btn + Vec2::new(50.0, 120.0)
                ),
            ],
            exit,
            exit_og,
            closed: false,
            close_time: 0.0,
        })
    }

    fn set_game_state(&mut self, factor: f64) {
        let change = -100.0 * factor;
        self.shopkeep.rect.x = change;
        self.bg.rect.y = change;
        for btn in self.tile_btns.iter_mut() {
            btn.set_pos(Vec2::new(0.0, change));
        }
        self.exit.set_pos(Vec2::new(self.exit_og.x, self.exit_og.y + change));
        self.fade.colour.a = ((1.0 - factor) * 100.0) as u8;
    }

    pub fn close_shop(&mut self) {
        self.closed = true;
    }

    pub fn update(&mut self, input: &Controls, ui: &mut Ui) {
        if self.first {
            self.first = false;
            for btn in self.tile_btns.iter_mut() {
                if !btn.set {
                    btn.set(ui);
                }
            }
        }
        
        if self.closed {
            self.close_time += input.frame_elapsed;
            let close_ratio = (self.close_time / CLOSE_TIME).powf(2.0);
            self.set_game_state(close_ratio);
            return;
        }
        
        self.since_opened += input.frame_elapsed;
        if self.since_opened < OPEN_TIME {
            let open_ratio = 1.0 - (self.since_opened / OPEN_TIME).powf(0.5);
            self.set_game_state(open_ratio);
        }
        let t = ui.pop_tile();
        if t != Tiles::None {
            for b in self.tile_btns.iter() {
                if b.t == t {
                    ui.set_money(ui.get_money() + b.cost);
                }
            }
        }
        for btn in self.tile_btns.iter_mut() {
            btn.btn.update(input);
            btn.btn.set_colour(if  ui.get_money() < btn.cost {
                Colour::new(150, 150, 150, 255)
            } else {
                Colour::white()
            });
            if btn.btn.clicked() {
                if ui.get_money() >= btn.cost {
                    ui.set_money(ui.get_money() - btn.cost);
                    ui.set_tile(btn.t);
                    self.closed = true;
                }
            }
        }
        self.exit.update(input);
        if self.exit.clicked() {
            self.closed = true;
        }
    }

    pub fn draw(&self, cam: &mut Camera) {
        cam.draw(&self.fade);
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
        self.close_time > CLOSE_TIME
    }

    pub fn open(&mut self) {
        self.since_opened = 0.0;
        self.close_time = 0.0;
        self.closed = false;
    }
}
