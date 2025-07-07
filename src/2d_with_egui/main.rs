//! Minimal raylib ✕ egui (OpenGL) bridge.
//! Tested with raylib-rs 5.5.1 and egui_glow 0.31.

use std::{ffi::CString, sync::Arc};

use egui::{Context, Event, PointerButton, RawInput};
use egui_glow::Painter;
use glam::{Mat2, Vec2};
use glow::HasContext;
use raylib::prelude::*;

mod sketch; // small self-contained state/update/draw module
use sketch::{draw, egui_ui, step, State};

fn main() {
    /* ---- boot raylib ---------------------------------------------------- */
    let (mut rl, th) = raylib::init()
        .size(1280, 720)
        .title("raylib + egui minimal template")
        .vsync()
        .build();
    unsafe { raylib::ffi::SetTraceLogLevel(raylib::consts::TraceLogLevel::LOG_WARNING as _) };

    /* ---- grab GL function loader for glow ------------------------------ */
    let gl = unsafe {
        glow::Context::from_loader_function(|s| {
            let c = CString::new(s).unwrap();
            raylib::ffi::rlGetProcAddress(c.as_ptr()) as *const _
        })
    };
    let painter =
        Painter::new(Arc::new(gl), "", None, false).expect("failed to create egui_glow painter");
    let mut painter = Some(painter); // will move into loop

    let mut egui_ctx = Context::default();
    let mut state = State::new();

    /* ---- main loop ------------------------------------------------------ */
    while state.running && !rl.window_should_close() {
        /* feed a *minimum* RawInput to egui ------------------------------ */
        let mut raw = RawInput::default();
        let mp = rl.get_mouse_position();
        raw.events.push(Event::PointerMoved([mp.x, mp.y].into()));
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            || rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
        {
            raw.events.push(Event::PointerButton {
                pos: [mp.x, mp.y].into(),
                button: PointerButton::Primary,
                pressed: rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
                modifiers: Default::default(),
            });
        }
        raw.screen_rect = Some(egui::Rect::from_min_size(
            [0.0, 0.0].into(),
            [rl.get_screen_width() as f32, rl.get_screen_height() as f32].into(),
        ));

        /* egui frame ------------------------------------------------------ */
        let out = egui_ctx.run(raw, |ctx| egui_ui(ctx, &mut state));

        /* fixed-timestep update ------------------------------------------ */
        let dt = rl.get_frame_time();
        step(&mut state, dt);

        /* normal raylib drawing ------------------------------------------ */
        let mut d = rl.begin_drawing(&th);
        d.clear_background(Color::BLACK);
        draw(&state, &mut d);

        /* paint egui on top ---------------------------------------------- */
        let dims = [rl.get_screen_width() as u32, rl.get_screen_height() as u32];
        let painter = painter.as_mut().unwrap();

        // upload any new textures
        for (id, delta) in &out.textures_delta.set {
            painter.set_texture(*id, delta);
        }

        // tessellate & paint
        let clipped = egui_ctx.tessellate(out.shapes, 1.0);
        unsafe { painter.gl().disable(glow::SCISSOR_TEST) }; // raylib left it enabled
        painter.paint_primitives(dims, egui_ctx.pixels_per_point(), &clipped);

        // cleanup freed textures
        for id in &out.textures_delta.free {
            painter.free_texture(*id);
        }
    }
}
