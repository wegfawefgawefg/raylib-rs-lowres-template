// src/2d_with_egui/sketch.rs
use egui;
use glam::{Mat2, Vec2};
use raylib::prelude::*;

pub const FPS: u32 = 60;

#[derive(Debug)]
pub struct State {
    pub running: bool,
    pub angle_deg: f32,
    pub speed_deg_per_s: f32,
    pub color: Color,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: true,
            angle_deg: 0.0,
            speed_deg_per_s: 120.0,
            color: Color::GREEN,
        }
    }
}

/* ----------- fixed-step update --------------------------------------- */
pub fn step(st: &mut State, dt: f32) {
    st.angle_deg = (st.angle_deg + st.speed_deg_per_s * dt) % 360.0;
}

/* ----------- raylib drawing ------------------------------------------ */
pub fn draw(st: &State, d: &mut RaylibDrawHandle) {
    let centre = Vec2::new(d.get_screen_width() as f32, d.get_screen_height() as f32) * 0.5;
    let rot = Mat2::from_angle(st.angle_deg.to_radians());
    let offset = Vec2::new(150.0, 0.0);

    // three squares 120° apart
    for i in 0..3 {
        let p = rot * offset.rotate((i as f32) * 2.094_395) + centre;
        d.draw_rectangle((p.x - 20.0) as i32, (p.y - 20.0) as i32, 40, 40, st.color);
    }
}

/* ----------- egui overlay ------------------------------------------- */
pub fn egui_ui(ctx: &egui::Context, st: &mut State) {
    egui::Window::new("Controls").show(ctx, |ui| {
        ui.add(egui::Slider::new(&mut st.speed_deg_per_s, 0.0..=360.0).text("spin speed (°/s)"));

        let mut rgb = [st.color.r, st.color.g, st.color.b];
        if ui.color_edit_button_srgb(&mut rgb).changed() {
            st.color = Color::new(rgb[0], rgb[1], rgb[2], 255);
        }

        if ui.button("Quit").clicked() {
            st.running = false;
        }
    });
}
