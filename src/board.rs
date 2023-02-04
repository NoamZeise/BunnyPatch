use crate::button::Button;
use crate::tiles::{Tiles, Tilemap, Tile, Choice, TILE};
use crate::ui::Ui;

use sdl_helper::geometry::Vec2;
use sdl_helper::GameObject;
use sdl_helper::input::{Controls, keyboard::Key, keyboard::MouseButton};
use sdl_helper::{map::Map, Error, Render, Camera};
use std::path::Path;

pub struct Board {
    map: Option<Map>,
    obj_map: Vec<Box<dyn Tile>>,
    pub board: Tilemap,
    cursor: GameObject,
    is_selected: bool,
    selected: (usize, usize),
    pub money: usize,
    pub dir_btns: [Button; 4],
}

impl Board {
    pub fn new(render: &mut Render) -> Result<Board, Error> {
        Ok(
            Board {
                board : Tilemap::new(
                    render.texture_manager.load(Path::new(
                        "resources/textures/tiles/game_tiles.png"))
                        ?),
                map: None,
                obj_map: Vec::new(),
                is_selected: false,
                selected: (0, 0),
                cursor: GameObject::new_from_tex(render.texture_manager.load(
                    Path::new("resources/textures/cursor.png")
                )?),
                money: 0,
                dir_btns: get_dir_btn(render)?,
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

    pub fn update(&mut self, input: &Controls, ui: &mut Ui) {
        if input.kbm.press(Key::N) {
            for t in self.obj_map.iter_mut() {
                t.update(&mut self.board);
                if t.removed() {
                    *t = self.board.set_tile_obj(Tiles::Grass, t.pos().0, t.pos().1);
                }
            }
            let choices: Vec<Choice> = self.board.map_updates
                .drain(0..self.board.map_updates.len()).collect();
            for c in choices {
                let current_tile = self.obj_map[c.i].tile();
                if current_tile != c.dst {
                    if current_tile == Tiles::Grass {
                        self.set(c);
                    } else  {
                        self.obj_map[c.i].interact(c.dst);
                        if self.obj_map[c.i].removed() {
                            self.set(c);
                        }
                    }
                }
            }
        }
        self.set_cursor(input.kbm.mouse_pos());
        if self.is_selected && input.kbm.mouse_press(MouseButton::Left)
            && ui.get_tile() != Tiles::None {
            let i = self.board.bi(
                self.selected.0, self.selected.1);
            if self.board.map[i] != Tiles::None {
                self.set(Choice { i,
                                  x: self.selected.0, y: self.selected.1,
                                  src: Tiles::None, dst: ui.pop_tile() });
            }
            }

        for d in self.dir_btns.iter_mut() {
            d.update(input);
        }
    }

    fn set_cursor(&mut self, pos: Vec2) {
        let x: i64 = (pos.x / TILE.x) as i64;
        let y: i64 = (pos.y / TILE.y) as i64;

        if x < 0 || y < 0 || x >= self.board.w as i64 || y >= self.board.h as i64 {
            self.is_selected = false;
            return;
        }
        self.is_selected = true;
        self.selected = (x as usize, y as usize);
        self.cursor.rect.x = self.selected.0 as f64 * TILE.x;
        self.cursor.rect.y = self.selected.1 as f64 * TILE.y;
    }

    fn set(&mut self, c: Choice) {
        //println!("x:{}, y:{}", c.x, c.y);
        self.obj_map[c.i] = self.board.set_tile_obj(c.dst, c.x, c.y);
        self.board.map[c.i] = c.dst;
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
        if self.is_selected {
            cam.draw(&self.cursor);
        }

        for d in self.dir_btns.iter() {
            d.draw(cam);
        }
        
    }

    fn draw_map(&self, cam: &mut Camera) {
        for obj in self.obj_map.iter() {
            obj.draw(cam);
        }
    }

    fn _debug_tilemap(&self, cam: &mut Camera) {
        for x in 0..self.board.w {
            for y in 0..self.board.h {
                let i = self.board.bi(x, y);
                if self.obj_map[i].tile() == Tiles::Grass {
                    let mut go = self.board.resources[&Tiles::None][0];
                    go.rect.x = x as f64 * TILE.x;
                    go.rect.y = y as f64 * TILE.y;
                    cam.draw(
                        &go
                    );
                }
            }
        }
    }

    pub fn unselect(&mut self) {
        self.is_selected = false;
    }

    pub fn set_cam(&self, cam: &mut Camera) {
        cam.set_offset(
            Vec2::new(
                (self.board.w as f64 * TILE.x / 2.0) - cam.get_view_size().x/2.0,
                (self.board.h as f64 * TILE.y / 2.0) - cam.get_view_size().y/2.0,
            )
        );
    }
    
    pub fn update_cam(&self, cam: &mut Camera, controls: &Controls) {
        let mut off = cam.get_offset();
        const CAM_SPEED: f64 = 150.0;
        
        for (i, d) in self.dir_btns.iter().enumerate() {
            if d.held() {
                match i {
                    0 => {off.y -= CAM_SPEED * controls.frame_elapsed; },
                    1 => {off.y += CAM_SPEED * controls.frame_elapsed; },
                    2 => {off.x -= CAM_SPEED * controls.frame_elapsed; },
                    3 => {off.x += CAM_SPEED * controls.frame_elapsed; },
                    _ => (),
                       
                }
            }
        }

        
        let mut pos = off;
        if pos.x < 0.0 || pos.x > self.board.w as f64 * TILE.x - cam.get_view_size().x {
            pos.x = cam.get_offset().x;
        }
        if pos.y < 0.0 || pos.y > self.board.h as f64 * TILE.y - cam.get_view_size().y {
            pos.y = cam.get_offset().y;
        }
        cam.set_offset(pos);
    }
}

const BTN_MID: Vec2 = Vec2::new(50.0, 250.0);

fn get_dir_btn(render: &mut Render) -> Result<[Button; 4], Error> {
    let t = render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_up.png")
    )?;
    let size = Vec2::new(t.width as f64, t.height as f64);
    Ok([
        Button::new(
            GameObject::new_from_tex(
                t
            ),
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_up_active.png")
                )?
            ),
            BTN_MID+ Vec2::new(0.0, - size.y)
        ),
                Button::new(
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_down.png")
                )?
            ),
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_down_active.png")
                )?
            ),
                    BTN_MID + Vec2::new(0.0, size.y)
                ),
                Button::new(
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_left.png")
                )?
            ),
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_left_active.png")
                )?
            ),
            BTN_MID+ Vec2::new(-size.x, 0.0)
                ),
                Button::new(
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_right.png")
                )?
            ),
            GameObject::new_from_tex(
                render.texture_manager.load(
                    Path::new("resources/textures/btn/dir_right_active.png")
                )?
            ),
            BTN_MID + Vec2::new(size.x, 0.0)
                ),

        ])
}
