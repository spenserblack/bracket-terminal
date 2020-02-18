use super::super::super::{Console, GameState, BTerm};
use super::VirtualKeyCode;
use pancurses::{endwin, initscr, noecho, Window};
use std::time::Instant;

pub fn main_loop<GS: GameState>(mut BTerm: BTerm, mut gamestate: GS) {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let dummy_shader = super::shader::Shader {};

    while !BTerm.quitting {
        let now_seconds = now.elapsed().as_secs();
        frames += 1;

        if now_seconds > prev_seconds {
            BTerm.fps = frames as f32 / (now_seconds - prev_seconds) as f32;
            frames = 0;
            prev_seconds = now_seconds;
        }

        let now_ms = now.elapsed().as_millis();
        if now_ms > prev_ms {
            BTerm.frame_time_ms = (now_ms - prev_ms) as f32;
            prev_ms = now_ms;
        }

        // Input
        BTerm.left_click = false;
        BTerm.key = None;
        BTerm.shift = false;
        BTerm.control = false;
        BTerm.alt = false;
        let input = BTerm.backend.platform.window.getch();
        if let Some(input) = input {
            //println!("{:?}", input);

            match input {
                pancurses::Input::KeyLeft => BTerm.key = Some(VirtualKeyCode::Left),
                pancurses::Input::KeyRight => BTerm.key = Some(VirtualKeyCode::Right),
                pancurses::Input::KeyUp => BTerm.key = Some(VirtualKeyCode::Up),
                pancurses::Input::KeyDown => BTerm.key = Some(VirtualKeyCode::Down),
                pancurses::Input::KeyHome => BTerm.key = Some(VirtualKeyCode::Home),
                pancurses::Input::KeyMouse => {
                    if let Ok(mouse_event) = pancurses::getmouse() {
                        if mouse_event.bstate & pancurses::BUTTON1_CLICKED > 0 {
                            BTerm.left_click = true;
                        }
                        BTerm.mouse_pos = (mouse_event.x, mouse_event.y);
                    }
                }
                pancurses::Input::Character(c) => match c {
                    '`' => BTerm.key = Some(VirtualKeyCode::Grave),
                    '1' => BTerm.key = Some(VirtualKeyCode::Key1),
                    '2' => BTerm.key = Some(VirtualKeyCode::Key2),
                    '3' => BTerm.key = Some(VirtualKeyCode::Key3),
                    '4' => BTerm.key = Some(VirtualKeyCode::Key4),
                    '5' => BTerm.key = Some(VirtualKeyCode::Key5),
                    '6' => BTerm.key = Some(VirtualKeyCode::Key6),
                    '7' => BTerm.key = Some(VirtualKeyCode::Key7),
                    '8' => BTerm.key = Some(VirtualKeyCode::Key8),
                    '9' => BTerm.key = Some(VirtualKeyCode::Key9),
                    '0' => BTerm.key = Some(VirtualKeyCode::Key0),
                    'a' => BTerm.key = Some(VirtualKeyCode::A),
                    'b' => BTerm.key = Some(VirtualKeyCode::B),
                    'c' => BTerm.key = Some(VirtualKeyCode::C),
                    'd' => BTerm.key = Some(VirtualKeyCode::D),
                    'e' => BTerm.key = Some(VirtualKeyCode::E),
                    'f' => BTerm.key = Some(VirtualKeyCode::F),
                    'g' => BTerm.key = Some(VirtualKeyCode::G),
                    'h' => BTerm.key = Some(VirtualKeyCode::H),
                    'i' => BTerm.key = Some(VirtualKeyCode::I),
                    'j' => BTerm.key = Some(VirtualKeyCode::J),
                    'k' => BTerm.key = Some(VirtualKeyCode::K),
                    'l' => BTerm.key = Some(VirtualKeyCode::L),
                    'm' => BTerm.key = Some(VirtualKeyCode::M),
                    'n' => BTerm.key = Some(VirtualKeyCode::N),
                    'o' => BTerm.key = Some(VirtualKeyCode::O),
                    'p' => BTerm.key = Some(VirtualKeyCode::P),
                    'q' => BTerm.key = Some(VirtualKeyCode::Q),
                    'r' => BTerm.key = Some(VirtualKeyCode::R),
                    's' => BTerm.key = Some(VirtualKeyCode::S),
                    't' => BTerm.key = Some(VirtualKeyCode::T),
                    'u' => BTerm.key = Some(VirtualKeyCode::U),
                    'v' => BTerm.key = Some(VirtualKeyCode::V),
                    'w' => BTerm.key = Some(VirtualKeyCode::W),
                    'x' => BTerm.key = Some(VirtualKeyCode::X),
                    'y' => BTerm.key = Some(VirtualKeyCode::Y),
                    'z' => BTerm.key = Some(VirtualKeyCode::Z),
                    '\t' => BTerm.key = Some(VirtualKeyCode::Tab),
                    '\n' => BTerm.key = Some(VirtualKeyCode::Return),
                    ',' => BTerm.key = Some(VirtualKeyCode::Comma),
                    '.' => BTerm.key = Some(VirtualKeyCode::Period),
                    '/' => BTerm.key = Some(VirtualKeyCode::Slash),
                    '[' => BTerm.key = Some(VirtualKeyCode::LBracket),
                    ']' => BTerm.key = Some(VirtualKeyCode::RBracket),
                    '\\' => BTerm.key = Some(VirtualKeyCode::Backslash),
                    _ => {}
                },
                _ => {}
            }
        }

        gamestate.tick(&mut BTerm);

        for cons in &mut BTerm.consoles {
            cons.console.rebuild_if_dirty(&BTerm.backend);
        }

        BTerm.backend.platform.window.clear();

        // Tell each console to draw itself
        for cons in &mut BTerm.consoles {
            cons.console
                .gl_draw(&BTerm.fonts[cons.font_index], &dummy_shader, &BTerm.backend);
        }

        BTerm.backend.platform.window.refresh();

        crate::hal::fps_sleep(BTerm.backend.platform.frame_sleep_time, &now, prev_ms);
    }

    endwin();
}
