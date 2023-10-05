use glam::Vec2;
use raylib::prelude::*;

pub const FRAMES_PER_SECOND: u32 = 60;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,

    pub camera: Camera3D,
}

impl State {
    pub fn new() -> Self {
        let camera = Camera3D::perspective(
            Vector3::new(4.0, 4.0, 4.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            60.0,
        );

        Self {
            running: true,
            time_since_last_update: 0.0,

            camera,
        }
    }
}

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }
}

pub fn step(_rl: &mut RaylibHandle, _rlt: &mut RaylibThread, _state: &mut State) {}

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Low Res Sketch!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    let mut d3 = d.begin_mode3D(state.camera);
    let plane_y = -3.0 + ((d3.get_time() as f32) * 1.0).sin() * 1.0;
    d3.draw_plane(
        Vector3::new(0.0, plane_y, 0.0),
        Vector2::new(6.0, 6.0),
        Color::LIGHTGRAY,
    );

    let angle = d3.get_time() as f32;
    let center = Vec2::new(0.0, 0.0) / 2.0;
    let offset = Vec2::new(10.0, 0.0) / 4.0;
    for i in 0..3 {
        let rot = glam::Mat2::from_angle(angle + i as f32 * 90.0);
        let rect_pos_rotated = rot * offset + center;

        let size = (((d3.get_time() as f32 + i as f32 * 1.0) * 2.0).sin() + 1.0) / 2.0 * 1.0 + 0.0;
        d3.draw_cube(
            Vector3::new(rect_pos_rotated.x, rect_pos_rotated.y, 0.0),
            size,
            size,
            size,
            Color::GOLD,
        );
    }
}
