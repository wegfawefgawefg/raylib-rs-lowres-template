// src/2d_with_egui/main.rs
//! minimal raylib ✕ egui example (raylib-rs 5.5.1 / egui_glow 0.31)

use std::{ffi::CString, sync::Arc};

use egui::{Event, PointerButton, Pos2, RawInput};
use egui_glow::Painter;
use glam::Vec2;
use glow::HasContext;
use raylib::prelude::*;

mod sketch;
use sketch::{draw, egui_ui, step, State};

fn main() {
    /* --- boot raylib --------------------------------------------------- */
    let (mut rl, th) = raylib::init()
        .size(1280, 720)
        .title("raylib + egui minimal")
        .vsync()
        .build();
    unsafe { raylib::ffi::SetTraceLogLevel(raylib::consts::TraceLogLevel::LOG_WARNING as _) };

    /* --- make sure rlgl has all GL entry-points ------------------------ */
    unsafe {
        // This call is what raylib itself does internally on desktop.
        // We repeat it so that *glow* can see the same GL symbols later on.
        raylib::ffi::rlLoadExtensions(Some(raylib::ffi::glfwGetProcAddress));
    }

    /* --- create a glow::Context that asks GLFW for symbols ------------ */
    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            let cs = CString::new(s).unwrap();
            unsafe { raylib::ffi::glfwGetProcAddress(cs.as_ptr()) as *const _ }
        })
    };
    let mut painter =
        Painter::new(Arc::new(gl), "", None, false).expect("could not create egui_glow painter");
    let mut egui_ctx = egui::Context::default();

    /* --- game state ---------------------------------------------------- */
    let mut state = State::new();

    /* --- main loop ----------------------------------------------------- */
    while state.running && !rl.window_should_close() {
        /* feed *minimal* input to egui ------------------------------- */
        let mut raw = RawInput::default();
        let mp = rl.get_mouse_position();
        raw.events
            .push(Event::PointerMoved(Pos2::new(mp.x as f32, mp.y as f32)));
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            || rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            raw.events.push(Event::PointerButton {
                pos: Pos2::new(mp.x as f32, mp.y as f32),
                button: PointerButton::Primary,
                pressed: rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
                modifiers: Default::default(),
            });
        }
        raw.screen_rect = Some(egui::Rect::from_min_size(
            [0.0, 0.0].into(),
            [rl.get_screen_width() as f32, rl.get_screen_height() as f32].into(),
        ));

        /* egui frame ------------------------------------------------- */
        let out = egui_ctx.run(raw, |ctx| egui_ui(ctx, &mut state));

        /* fixed-step update ----------------------------------------- */
        let dt = rl.get_frame_time();
        step(&mut state, dt);

        /* raylib drawing -------------------------------------------- */
        let mut d = rl.begin_drawing(&th);
        d.clear_background(Color::BLACK);
        draw(&state, &mut d);

        /* paint egui on top ----------------------------------------- */
        let dims = [rl.get_screen_width() as u32, rl.get_screen_height() as u32];

        // upload textures that egui asked for
        for (id, delta) in &out.textures_delta.set {
            painter.set_texture(*id, delta);
        }

        let clipped = egui_ctx.tessellate(out.shapes, 1.0);

        // raylib left scissor test enabled – egui disables it for its own pass
        unsafe { painter.gl().disable(glow::SCISSOR_TEST) };
        painter.paint_primitives(dims, egui_ctx.pixels_per_point(), &clipped);

        // remove textures that egui wants to free
        for id in &out.textures_delta.free {
            painter.free_texture(*id);
        }
    }
}
