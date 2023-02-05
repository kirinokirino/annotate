#![warn(clippy::nursery, clippy::pedantic)]

use speedy2d::{
    color::Color,
    dimen::{UVec2, Vec2},
    window::{
        KeyScancode, MouseButton, VirtualKeyCode, WindowCreationOptions, WindowHandler,
        WindowHelper, WindowPosition, WindowSize,
    },
    Graphics2D, Window,
};

mod history;
use history::History;

const WINDOW_WIDTH: u32 = 1366;
const WINDOW_HEIGHT: u32 = 768;

fn main() {
    let window_size = UVec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        "FLOATING",
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(false)
            .with_transparent(true),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size));
}
struct App {
    window_size: UVec2,
    lmb: bool,
    should_clear: bool,
    mouse_positions: History,
}

impl App {
    pub fn new(window_size: UVec2) -> Self {
        Self {
            window_size,
            lmb: false,
            should_clear: false,
            mouse_positions: History::new(16),
        }
    }
}

impl WindowHandler for App {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        let spread = 7.0;
        if self.should_clear {
            graphics.clear_screen(Color::from_rgba(0.0, 0.0, 0.0, 0.0));
            self.should_clear = false;
        }
        if self.lmb {
            let mut next = None;
            for history in &self.mouse_positions {
                if let Some(next) = next {
                    if let Some(history) = history {
                        graphics.draw_line(
                            history + Vec2::new(fastrand::f32() * spread, fastrand::f32() * spread),
                            next + Vec2::new(fastrand::f32() * spread, fastrand::f32() * spread),
                            2.0,
                            Color::BLACK,
                        );
                    }
                }
                next = history;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
        helper.request_redraw();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.window_size = size_pixels;
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        if self.lmb {
            self.mouse_positions.push(Some(position));
        } else {
            self.mouse_positions.push(None);
        }
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        if button == MouseButton::Left {
            self.lmb = true;
        }
    }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<()>, button: MouseButton) {
        if button == MouseButton::Left {
            self.lmb = false;
        }
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            match key_code {
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::LControl => self.should_clear = true,
                key => println!("Key: {key:?}, scancode: {scancode}"),
            }
        }
    }
}
