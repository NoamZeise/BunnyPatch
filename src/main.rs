use std::path::Path;
use sdl_helper::{input::Controls, Render, DrawingArea, Error, input::keyboard::Key, geometry::*, Camera};
use ggj2023::board::Board;


pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Name of Game",                              // window name
        Rect::new(0.0, 0.0, 240.0, 160.0), // window camera
        Vec2::new(480.0, 320.0)            // window size
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;
    
    let mut board = Board::new(&mut render)?;

    board.load_map(Path::new("resources/maps/env.tmx"), &mut render)?;
    
    while !controls.should_close {
        controls.update(&cam);
        
        board.update(&controls);

        if controls.kbm.down(Key::Escape) {
            controls.should_close = true;
        }

        if controls.kbm.down(Key::Equals) {
            increase_win_size(&mut cam, &mut render)?;
        }

        if controls.kbm.down(Key::Minus) {
            reduce_win_size(&mut cam, &mut render)?;
        }
        
        render.start_draw();

        board.draw(&mut cam);
        
        render.end_draw(&mut cam)?;
    }
    Ok(())
}


fn increase_win_size(cam: &mut Camera, render: &mut Render) -> Result<(), Error> {
    let mut cs = cam.get_window_size();
    if cs.x < cam.get_view_size().x {
        cs.x *= 2.0;
        cs.y *= 2.0;
    } else {
        cs.x += cam.get_view_size().x/2.0;
        cs.y += cam.get_view_size().y/2.0;
    }
    render.set_win_size(cam, cs, false)
}

fn reduce_win_size(cam: &mut Camera, render: &mut Render) -> Result<(), Error> {
    let mut cs = cam.get_window_size();
    if cs.x <= cam.get_view_size().x {
        cs.x /= 2.0;
        cs.y /= 2.0;
    } else {
        cs.x -= cam.get_view_size().x/2.0;
        cs.y -= cam.get_view_size().y/2.0;
    }
    render.set_win_size(cam, cs, false)
}
