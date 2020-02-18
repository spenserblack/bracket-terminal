use super::super::super::{GameState, BTerm};
use super::VirtualKeyCode;
use crossterm::event::{poll, read, Event};
use crossterm::execute;
use crossterm::terminal::SetSize;
use std::io::{stdout, Write};
use std::time::Duration;
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

        // Input handler goes here
        while poll(Duration::from_secs(0)).unwrap() {
            match read().expect("Uh oh") {
                Event::Mouse(event) => {
                    //println!("{:?}", event);
                    // Button capture goes here
                    // Mouse doesn't seem to support cursor position? That's going to cause issues.
                    match event {
                        crossterm::event::MouseEvent::Down(_button, x, y, _modifiers) => {
                            BTerm.left_click = true;
                            BTerm.mouse_pos = (x as i32 * 8, y as i32 * 8);
                        }
                        _ => {}
                    }
                }
                Event::Key(key) => {
                    // Including because it eats my ctrl-C to quit!
                    if key.code == crossterm::event::KeyCode::Char('c')
                        && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                    {
                        BTerm.quitting = true;
                    }

                    use crossterm::event::KeyCode;
                    match key.code {
                        KeyCode::Left => BTerm.key = Some(VirtualKeyCode::Left),
                        KeyCode::Right => BTerm.key = Some(VirtualKeyCode::Right),
                        KeyCode::Up => BTerm.key = Some(VirtualKeyCode::Up),
                        KeyCode::Down => BTerm.key = Some(VirtualKeyCode::Down),
                        KeyCode::Backspace => BTerm.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Enter => BTerm.key = Some(VirtualKeyCode::Return),
                        KeyCode::Home => BTerm.key = Some(VirtualKeyCode::Home),
                        KeyCode::End => BTerm.key = Some(VirtualKeyCode::End),
                        KeyCode::PageUp => BTerm.key = Some(VirtualKeyCode::PageUp),
                        KeyCode::PageDown => BTerm.key = Some(VirtualKeyCode::PageDown),
                        KeyCode::Tab => BTerm.key = Some(VirtualKeyCode::Tab),
                        KeyCode::Delete => BTerm.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Insert => BTerm.key = Some(VirtualKeyCode::Insert),
                        KeyCode::Esc => BTerm.key = Some(VirtualKeyCode::Escape),
                        KeyCode::F(1) => BTerm.key = Some(VirtualKeyCode::F1),
                        KeyCode::F(2) => BTerm.key = Some(VirtualKeyCode::F2),
                        KeyCode::F(3) => BTerm.key = Some(VirtualKeyCode::F3),
                        KeyCode::F(4) => BTerm.key = Some(VirtualKeyCode::F4),
                        KeyCode::F(5) => BTerm.key = Some(VirtualKeyCode::F5),
                        KeyCode::F(6) => BTerm.key = Some(VirtualKeyCode::F6),
                        KeyCode::F(7) => BTerm.key = Some(VirtualKeyCode::F7),
                        KeyCode::F(8) => BTerm.key = Some(VirtualKeyCode::F8),
                        KeyCode::F(9) => BTerm.key = Some(VirtualKeyCode::F9),
                        KeyCode::F(10) => BTerm.key = Some(VirtualKeyCode::F10),
                        KeyCode::F(11) => BTerm.key = Some(VirtualKeyCode::F11),
                        KeyCode::F(12) => BTerm.key = Some(VirtualKeyCode::F12),
                        KeyCode::Char('`') => BTerm.key = Some(VirtualKeyCode::Grave),
                        KeyCode::Char('1') => BTerm.key = Some(VirtualKeyCode::Key1),
                        KeyCode::Char('2') => BTerm.key = Some(VirtualKeyCode::Key2),
                        KeyCode::Char('3') => BTerm.key = Some(VirtualKeyCode::Key3),
                        KeyCode::Char('4') => BTerm.key = Some(VirtualKeyCode::Key4),
                        KeyCode::Char('5') => BTerm.key = Some(VirtualKeyCode::Key5),
                        KeyCode::Char('6') => BTerm.key = Some(VirtualKeyCode::Key6),
                        KeyCode::Char('7') => BTerm.key = Some(VirtualKeyCode::Key7),
                        KeyCode::Char('8') => BTerm.key = Some(VirtualKeyCode::Key8),
                        KeyCode::Char('9') => BTerm.key = Some(VirtualKeyCode::Key9),
                        KeyCode::Char('0') => BTerm.key = Some(VirtualKeyCode::Key0),
                        KeyCode::Char('-') => BTerm.key = Some(VirtualKeyCode::Minus),
                        KeyCode::Char('=') => BTerm.key = Some(VirtualKeyCode::Equals),
                        KeyCode::Char('a') => BTerm.key = Some(VirtualKeyCode::A),
                        KeyCode::Char('b') => BTerm.key = Some(VirtualKeyCode::B),
                        KeyCode::Char('c') => BTerm.key = Some(VirtualKeyCode::C),
                        KeyCode::Char('d') => BTerm.key = Some(VirtualKeyCode::D),
                        KeyCode::Char('e') => BTerm.key = Some(VirtualKeyCode::E),
                        KeyCode::Char('f') => BTerm.key = Some(VirtualKeyCode::F),
                        KeyCode::Char('g') => BTerm.key = Some(VirtualKeyCode::G),
                        KeyCode::Char('h') => BTerm.key = Some(VirtualKeyCode::H),
                        KeyCode::Char('i') => BTerm.key = Some(VirtualKeyCode::I),
                        KeyCode::Char('j') => BTerm.key = Some(VirtualKeyCode::J),
                        KeyCode::Char('k') => BTerm.key = Some(VirtualKeyCode::K),
                        KeyCode::Char('l') => BTerm.key = Some(VirtualKeyCode::L),
                        KeyCode::Char('m') => BTerm.key = Some(VirtualKeyCode::M),
                        KeyCode::Char('n') => BTerm.key = Some(VirtualKeyCode::N),
                        KeyCode::Char('o') => BTerm.key = Some(VirtualKeyCode::O),
                        KeyCode::Char('p') => BTerm.key = Some(VirtualKeyCode::P),
                        KeyCode::Char('q') => BTerm.key = Some(VirtualKeyCode::Q),
                        KeyCode::Char('r') => BTerm.key = Some(VirtualKeyCode::R),
                        KeyCode::Char('s') => BTerm.key = Some(VirtualKeyCode::S),
                        KeyCode::Char('t') => BTerm.key = Some(VirtualKeyCode::T),
                        KeyCode::Char('u') => BTerm.key = Some(VirtualKeyCode::U),
                        KeyCode::Char('v') => BTerm.key = Some(VirtualKeyCode::V),
                        KeyCode::Char('w') => BTerm.key = Some(VirtualKeyCode::W),
                        KeyCode::Char('x') => BTerm.key = Some(VirtualKeyCode::X),
                        KeyCode::Char('y') => BTerm.key = Some(VirtualKeyCode::Y),
                        KeyCode::Char('z') => BTerm.key = Some(VirtualKeyCode::Z),
                        KeyCode::Char('[') => BTerm.key = Some(VirtualKeyCode::LBracket),
                        KeyCode::Char(']') => BTerm.key = Some(VirtualKeyCode::RBracket),
                        KeyCode::Char('\\') => BTerm.key = Some(VirtualKeyCode::Backslash),
                        KeyCode::Char(';') => BTerm.key = Some(VirtualKeyCode::Semicolon),
                        KeyCode::Char('\'') => BTerm.key = Some(VirtualKeyCode::Apostrophe),
                        KeyCode::Char(',') => BTerm.key = Some(VirtualKeyCode::Comma),
                        KeyCode::Char('.') => BTerm.key = Some(VirtualKeyCode::Period),
                        KeyCode::Char('/') => BTerm.key = Some(VirtualKeyCode::Slash),

                        _ => {}
                    }

                    // Modifier handling
                    if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                        BTerm.control = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::SHIFT {
                        BTerm.shift = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::ALT {
                        BTerm.alt = true;
                    }
                }
                _ => {}
            }
        }

        gamestate.tick(&mut BTerm);

        for cons in &mut BTerm.consoles {
            cons.console.rebuild_if_dirty(&BTerm.backend);
        }

        // Tell each console to draw itself
        for cons in &mut BTerm.consoles {
            cons.console
                .gl_draw(&BTerm.fonts[cons.font_index], &dummy_shader, &BTerm.backend);
        }

        //BTerm.backend.platform.window.refresh();
        stdout().flush().expect("Command fail");

        crate::hal::fps_sleep(BTerm.backend.platform.frame_sleep_time, &now, prev_ms);
    }

    println!(
        "Returning size to {}x{}",
        BTerm.backend.platform.old_width, BTerm.backend.platform.old_height
    );
    execute!(
        stdout(),
        SetSize(
            BTerm.backend.platform.old_width,
            BTerm.backend.platform.old_height
        )
    )
    .expect("Unable to resize");
    execute!(
        stdout(),
        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
            r: 255,
            g: 255,
            b: 255
        })
    )
    .expect("Unable to recolor");
    execute!(
        stdout(),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 })
    )
    .expect("Unable to recolor");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
}
