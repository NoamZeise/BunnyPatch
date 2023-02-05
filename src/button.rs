use sdl_helper::{GameObject, geometry::{Vec2, Rect}, input::Controls, Camera, Colour};

pub struct Button {
    normal: GameObject,
    active: GameObject,
    selected: bool,
    clicked: bool,
    held: bool,
}

impl Button {
    pub fn new(normal: GameObject, active: GameObject, pos: Vec2) -> Button {
        let mut normal = normal;
        let mut active  = active;
        normal.rect.x = pos.x;
        normal.rect.y = pos.y;
        normal.parallax = Vec2::new(0.0, 0.0);
        active.rect.x = pos.x;
        active.rect.y = pos.y;
        active.parallax = Vec2::new(0.0, 0.0);
        Button {
            normal,
            active,
            selected: false,
            clicked: false,
            held: false,
        }
    }

    pub fn update(&mut self, control: &Controls) {
        self.selected = self.normal.rect.contains(&control.kbm.mouse_pos_cam_off());
        self.clicked = self.selected && control.kbm.mouse_press(sdl_helper::input::keyboard::MouseButton::Left);
        self.held = self.selected && control.kbm.mouse_hold(sdl_helper::input::keyboard::MouseButton::Left);
    }

    pub fn draw(&self, cam: &mut Camera) {
        if self.selected {
            cam.draw(&self.active);
        } else {
            cam.draw(&self.normal);
        }
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn held(&self) -> bool {
        self.held
    }

    pub fn set_colour(&mut self, col: Colour) {
        self.active.colour = col;
        self.normal.colour = col;
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.normal.rect.x = pos.x;
        self.normal.rect.y = pos.y;
        self.active.rect.x = pos.x;
        self.active.rect.y = pos.y;
    }

    pub fn get_rect(&self) -> Rect {
        self.normal.rect
    }
}

