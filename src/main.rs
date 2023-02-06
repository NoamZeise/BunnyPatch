use std::path::Path;
use sdl_helper::{input::{Controls, controller::Button}, Render, DrawingArea, Error, input::keyboard::Key, geometry::*, Camera, GameObject, audio::AudioManager, Colour};
use bunny_patch::{board::Board, shop::Shop, button, ui::Ui, tiles::Tilemap};

#[derive (Eq, PartialEq)]
enum GameState {
    Board,
    Shop,
    Lose,
    Win,
}

const DEATH_TIME: f64 = 1.2;
const WIN_FADE: f64 = 1.5;

pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Bunny Veg",                              // window name
        Rect::new(0.0, 0.0, 480.0, 320.0), // window camera
        Vec2::new(480.0 * 2.0, 320.0 * 2.0)            // window size
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;

    let mut audio = AudioManager::new()?;
    let m = audio.music.load(Path::new("resources/audio/Noam_Roots_Ambient1.mp3"))?;
    audio.music.play(m, -1)?;

    let lose_sfx = audio.sfx.load(Path::new("resources/audio/fail.mp3"))?;
    let win_sfx = audio.sfx.load(Path::new("resources/audio/end.mp3"))?;
    

    let mut game_state = GameState::Board;
    
    let mut board = Board::new(&mut render)?;
    board.load_map(Path::new("resources/maps/env.tmx"), &mut render)?;
    board.set_cam(&mut cam);

    let mut shop = Shop::new(&mut render)?;

    let mut ui = Ui::new(&mut render)?;

    let mut shop_btn = button::Button::new(
        GameObject::new_from_tex(render.texture_manager.load(
            Path::new("resources/textures/btn/shop.png")
        )?),
        GameObject::new_from_tex(render.texture_manager.load(
            Path::new("resources/textures/btn/shop_active.png")
        )?),
        Vec2::new(20.0, 10.0)
    );
    let pm_tex = render.texture_manager.load(Path::new("resources/textures/btn/change.png"))?;
    let mut plus_btn = button::Button::new(
        Tilemap::get_tile(pm_tex, 0, 0),
        Tilemap::get_tile(pm_tex, 0, 1),
        Vec2::new(70.0, 10.0)
    );
    let mut minus_btn = button::Button::new(
        Tilemap::get_tile(pm_tex, 1, 0),
        Tilemap::get_tile(pm_tex, 1, 1),
        Vec2::new(70.0, 30.0)
    );

    let mut fade = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/tiles/blank.png"))?);
    fade.colour = Colour::new(10, 10, 10, 0);
    fade.rect.w = 500.0;
    fade.rect.h = 500.0;
    fade.parallax = Vec2::new(0.0, 0.0);

    let mut win = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/win_screen.png"))?);
    win.parallax = Vec2::new(0.0, 0.0);
    let mut fade_done = false;
    let mut fade_in = true;
    let mut fade_time = 0.0;
    
    while !controls.should_close {
        controls.update(&cam);

        shop_btn.update(&controls);
        if shop_btn.clicked() || controls.kbm.press(Key::Escape) {
            match game_state {
                GameState::Board => {
                    game_state = GameState::Shop;
                    shop.open();
                }
                GameState::Shop => {
                    shop.close_shop();
                }
                _ => (),
            }
        }

        ui.update(&controls);

        match game_state {
            GameState::Board => {
                board.update(&controls, &mut ui);
                board.update_cam(&mut cam, &controls);
                if board.lose {
                    game_state = GameState::Lose;
                    audio.sfx.play(lose_sfx)?;
                }
                if board.complete {
                    ui.pop_tile();
                    game_state = GameState::Win;
                    audio.sfx.play(win_sfx)?;
                }
            },
            GameState::Shop => {
                shop.update(&controls, &mut ui);
                if shop.change() {
                    game_state = GameState::Board;
                }
            },
            GameState::Lose => {
                fade_time += controls.frame_elapsed;
                fade.colour.a = ((fade_time  / DEATH_TIME) * 100.0) as u8;
                if fade.colour.a > 200 {
                    break;
                }
            },
            GameState::Win => {
                if controls.kbm.mouse_press(sdl_helper::input::keyboard::MouseButton::Left) {
                    break;
                }
                if !fade_done {
                    fade_time += controls.frame_elapsed;
                    if fade_in {
                        fade.colour.a = ((fade_time  / WIN_FADE) * 255.0) as u8;
                    } else {
                        fade.colour.a = ((1.0 -(fade_time  / WIN_FADE)) * 255.0) as u8;
                    }
                    if (fade_in && fade.colour.a >= 255) ||
                    (!fade_in && fade.colour.a < 10) {
                        if !fade_in {
                            fade_done = true;
                        } else {
                            fade_in = false;
                            fade_time = 0.0;
                        }
                    }
                }
            }
        }

        plus_btn.update(&controls);
        if controls.kbm.press(Key::Equals) || plus_btn.clicked() {
            increase_win_size(&mut cam, &mut render)?;
        }
        minus_btn.update(&controls);
        if controls.kbm.press(Key::Minus) || minus_btn.clicked() {
            reduce_win_size(&mut cam, &mut render)?;
        }
        
        render.start_draw();

        board.draw(&mut cam);

        
        shop_btn.draw(&mut cam);
        minus_btn.draw(&mut cam);
        plus_btn.draw(&mut cam);

        if game_state == GameState::Shop {
            shop.draw(&mut cam);
        }
        
        ui.draw(&mut cam);

        if game_state == GameState::Lose {
            cam.draw(&fade);
        }

        if game_state == GameState::Win {
            cam.draw(&fade);
            if !fade_in {
                cam.draw(&win);
            }
        }
        
        render.end_draw(&mut cam)?;
    }
    Ok(())
}


fn increase_win_size(cam: &mut Camera, render: &mut Render) -> Result<(), Error> {
    let mut cs = cam.get_window_size();
    if cs.x < cam.get_view_size().x {
        cs.x *= 4.0;
        cs.y *= 4.0;
    } else {
        cs.x += (cam.get_view_size().x/2.0) * 2.0;
        cs.y += (cam.get_view_size().y/2.0) * 2.0;
    }
    render.set_win_size(cam, cs, false)
}

fn reduce_win_size(cam: &mut Camera, render: &mut Render) -> Result<(), Error> {
    let mut cs = cam.get_window_size();
    if cs.x <= cam.get_view_size().x {
        cs.x /= 4.0;
        cs.y /= 4.0;
    } else {
        cs.x -= (cam.get_view_size().x/2.0) * 2.0;
        cs.y -= (cam.get_view_size().y/2.0) * 2.0;
    }
    render.set_win_size(cam, cs, false)
}
