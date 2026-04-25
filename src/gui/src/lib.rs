use ninjalib::ninjalib::NinjaFile;
use sdl3::mouse::MouseButton;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::time::Duration;

struct WindowSize {
    x: u64,
    y: u64,
    w: u32,
    h: u32,
    block_h: u32,
    block_delta: u32,
    k: i32,
}

fn get_scale(ws: &WindowSize) -> f32 {
    let base = 10.0f32;
    base.powf(ws.k as f32 / 10.0)
}

pub fn run_window(ninja: NinjaFile) {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ninja log viz", 960, 540).resizable().position_centered().build().unwrap();

    let mut canvas = window.into_canvas();
    let mut win_size = WindowSize {
        x: 0,
        y: 0,
        w: 960,
        h: 540,
        block_delta: 10,
        block_h: 30,
        k: 1,
    };

    let shift = 50i32;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_down = false;
    let mut pos_prev_x = -1i32;
    let mut pos_prev_y = -1i32;
    let mut pos_x = -1i32;
    let mut pos_y = -1i32;

    'running: loop {
        (win_size.w, win_size.h) = canvas.output_size().unwrap();
        let scale = get_scale(&win_size);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        // Выход по клавише Escape
                        Some(Keycode::KpPlus) => win_size.k = if win_size.k > 19 { 20 } else { win_size.k + 1 },
                        Some(Keycode::KpMinus) => win_size.k = if win_size.k < -19 { -20 } else { win_size.k - 1 },
                        Some(Keycode::Home) => {
                            win_size.x = 0;
                            win_size.y = 0;
                            is_down = false;
                        }
                        _ => {}
                    }
                    // println!("k: {}", win_size.k);
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            if is_down == false {
                                is_down = true;
                                pos_prev_x = win_size.x as i32;
                                pos_prev_y = win_size.y as i32;
                                pos_x = x as i32;
                                pos_y = y as i32;
                            }
                        }
                        _ => {}
                    }
                    // println!("x, y: {}, {}", win_size.x, win_size.y);
                }
                Event::MouseMotion { x, y, .. } => {
                    if is_down {
                        let shift_x = (pos_x as f32 - x) as i32;
                        let shift_y = (pos_y as f32 - y) as i32;

                        if shift_x >= 0 {
                            win_size.x = (pos_prev_x + shift_x) as u64;
                        } else {
                            let a = shift_x.abs() as i32;
                            win_size.x = if pos_prev_x > a { (pos_prev_x - a) as u64 } else { 0 };
                        }

                        if shift_y >= 0 {
                            win_size.y = (pos_prev_y + shift_y) as u64;
                        } else {
                            let a = shift_y.abs() as i32;
                            win_size.y = if pos_prev_y > a { (pos_prev_y - a) as u64 } else { 0 };
                        }
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        is_down = false;
                    }
                }
                _ => {}
            }
        }

        // render

        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for rec in ninja.records.iter() {
            let x = shift + (rec.start as f32 * scale) as i32 - win_size.x as i32;
            let y = shift + (rec.thread_id * (win_size.block_h + win_size.block_delta)) as i32 - win_size.y as i32;
            let w = (rec.dur as f32 * scale) as u32;
            let h = (win_size.block_h) as u32;

            canvas.draw_rect(Rect::new(x, y, w, h)).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_0() {
        let ws = WindowSize {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
            block_delta: 1,
            block_h: 1,
            k: 0,
        };
        let result = get_scale(&ws);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn scale_10() {
        let ws = WindowSize {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
            block_delta: 1,
            block_h: 1,
            k: 10,
        };
        let result = get_scale(&ws);
        assert_eq!(result, 10.0);
    }
}
