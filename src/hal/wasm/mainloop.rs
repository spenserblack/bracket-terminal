use super::super::super::{GameState, BTerm};
use super::events::*;
use glow::HasContext;

pub fn main_loop<GS: GameState>(mut BTerm: BTerm, mut gamestate: GS) {
    use glow::HasRenderLoop;

    let now = wasm_timer::Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let render_loop = glow::RenderLoop::from_request_animation_frame();
    render_loop.run(move |_running: &mut bool| {
        // Read in event results
        unsafe {
            BTerm.key = GLOBAL_KEY;
            BTerm.mouse_pos = (GLOBAL_MOUSE_POS.0, GLOBAL_MOUSE_POS.1);
            BTerm.left_click = GLOBAL_LEFT_CLICK;
            BTerm.shift = GLOBAL_MODIFIERS.0;
            BTerm.control = GLOBAL_MODIFIERS.1;
            BTerm.alt = GLOBAL_MODIFIERS.2;
            BTerm.web_button = GLOBAL_BUTTON.clone();
        }

        // Call the tock function
        tock(
            &mut BTerm,
            &mut gamestate,
            &mut frames,
            &mut prev_seconds,
            &mut prev_ms,
            &now,
        );

        // Clear any input
        BTerm.left_click = false;
        BTerm.key = None;
        unsafe {
            GLOBAL_KEY = None;
            GLOBAL_MODIFIERS = (false, false, false);
            GLOBAL_LEFT_CLICK = false;
            GLOBAL_BUTTON = None;
        }
    });
}

fn tock<GS: GameState>(
    BTerm: &mut BTerm,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &wasm_timer::Instant,
) {
    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        BTerm.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        BTerm.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(BTerm);

    // Console structure - doesn't really have to be every frame...
    for cons in &mut BTerm.consoles {
        cons.console.rebuild_if_dirty(&BTerm.backend);
    }

    // Clear the screen
    unsafe {
        BTerm.backend.platform.gl.viewport(
            0,
            0,
            BTerm.width_pixels as i32,
            BTerm.height_pixels as i32,
        );
        BTerm.backend.platform.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        BTerm.backend.platform.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Setup render pass
    let gl = &BTerm.backend.platform.gl;

    unsafe {
        BTerm.shaders[0].useProgram(gl);

        gl.active_texture(glow::TEXTURE0);
        BTerm.fonts[0].bind_texture(&BTerm.backend);
        BTerm.shaders[0].setInt(gl, "texture1", 0);
        BTerm.shaders[0].setVec3(gl, "font", 8.0, 8.0, 0.0);

        gl.bind_vertex_array(Some(BTerm.backend.platform.quad_vao));
    }

    // Tell each console to draw itself
    for cons in &mut BTerm.consoles {
        let font = &BTerm.fonts[cons.font_index];
        let shader = &BTerm.shaders[0];
        unsafe {
            gl.active_texture(glow::TEXTURE0);
            font.bind_texture(&BTerm.backend);
            shader.setBool(gl, "showScanLines", BTerm.post_scanlines);
            shader.setBool(gl, "screenBurn", BTerm.post_screenburn);
            shader.setVec3(
                gl,
                "screenSize",
                BTerm.width_pixels as f32,
                BTerm.height_pixels as f32,
                0.0,
            );
        }
        cons.console.gl_draw(font, shader, &BTerm.backend);
    }
}
