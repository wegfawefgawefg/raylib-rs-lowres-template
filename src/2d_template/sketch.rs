use glam::Vec2;
use raylib::prelude::*;

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: true,
            time_since_last_update: 0.0,
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

pub fn step(rl: &mut RaylibHandle, rlt: &mut RaylibThread, state: &mut State) {}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Low Res Sketch!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let angle = d.get_time() as f32;

    let center = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) / 2.0;
    let offset = center / 4.0;

    for i in 0..3 {
        let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
        let rect_pos_rotated = rot * offset + center;

        let size =
            (((d.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * offset.y + 4.0;
        d.draw_rectangle(
            rect_pos_rotated.x as i32,
            rect_pos_rotated.y as i32,
            size as i32,
            size as i32,
            Color::RED,
        );
    }
}
