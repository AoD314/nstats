use ninjalib::ninjalib::NinjaFile;
use sdl3::mouse::MouseButton;
use sdl3::render::{Canvas, FPoint};
use sdl3::video::Window;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use sdl3_ttf_sys::ttf::{TTF_Font, TTF_TextEngine};
use sdl3_ttf_sys::*;
use std::time::{Duration, Instant};

struct WindowSize {
    x: u32,
    y: u32,
    w: u32,
    h: u32,

    k: u8,           // ms in 100px
    block_delta: u8, // space between lines in px
    block_h: u16,    // height line in px
}

struct RectFile {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
}

fn get_scale(ws: &WindowSize) -> f32 {
    100.0_f32 / (1 << ws.k) as f32
}

fn draw_cell(canvas: &mut Canvas<Window>, ws: &WindowSize, engine: *mut TTF_TextEngine, font: *mut TTF_Font) {
    // Measuring scale

    // set cell color
    canvas.set_draw_color(Color::RGB(50, 50, 50));

    let delta = ws.x % 100;
    let count = ws.w / 100 + 1;
    let step = 1u32 << ws.k;
    let mut value: u32 = step * (ws.x / 100) as u32;

    for i in 0..count {
        let start = FPoint {
            x: -(delta as f32) + i as f32 * 100.0,
            y: 0.0,
        };
        let end = FPoint {
            x: -(delta as f32) + i as f32 * 100.0,
            y: ws.h as f32,
        };
        canvas.draw_line(start, end).unwrap();

        let txt = format!("{:?} ms", value);
        let text_len = txt.len();
        let text = txt.as_ptr();

        let x = 105.0_f32 - delta as f32 + i as f32 * 100.0;
        let y = 16.0_f32;
        value += step;

        unsafe {
            let text = ttf::TTF_CreateText(engine, font, text as *const i8, text_len);
            ttf::TTF_DrawRendererText(text, x, y);
            ttf::TTF_DestroyText(text);
        }
    }
}

pub fn run_window(ninja: NinjaFile) {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ninja log viz", 960, 540).resizable().position_centered().build().unwrap();

    let mut canvas = window.into_canvas();

    let renderer = canvas.raw();

    unsafe {
        if !ttf::TTF_Init() {
            println!("Fail initialization of SDL_ttf");
        }
    }

    let engine = unsafe { ttf::TTF_CreateRendererTextEngine(renderer) };

    let font_path = "font/jb.ttf\0";
    let font_size = 12;

    let font = unsafe {
        let f = ttf::TTF_OpenFont(font_path.as_ptr() as *const i8, font_size as f32);
        if f.is_null() {
            println!("Fail to load font");
            println!("{:?}", sdl3::get_error());
        }
        f
    };

    let mut win_size = WindowSize {
        x: 0,
        y: 0,
        w: 960,
        h: 540,
        block_delta: 10,
        block_h: 20,
        k: 1,
    };

    let shift: i32 = 100;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut is_down = false;
    let mut pos_prev_x = -1i32;
    let mut pos_prev_y = -1i32;
    let mut pos_x = -1i32;
    let mut pos_y = -1i32;
    let mut iter = 1;

    'running: loop {
        iter = (iter + 1) % 600;
        let timer = Instant::now();

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
                        Some(Keycode::KpPlus) => {
                            win_size.block_h += 2;
                            win_size.block_h = if win_size.block_h > 64 { 64 } else { win_size.block_h };
                            win_size.block_delta = (win_size.block_h >> 1) as u8;
                        }
                        Some(Keycode::KpMinus) => {
                            win_size.block_h -= 2;
                            win_size.block_h = if win_size.block_h < 8 { 8 } else { win_size.block_h };
                            win_size.block_delta = (win_size.block_h >> 1) as u8;
                        }
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
                            win_size.x = (pos_prev_x + shift_x) as u32;
                        } else {
                            let a = shift_x.abs() as i32;
                            win_size.x = if pos_prev_x > a { (pos_prev_x - a) as u32 } else { 0 };
                        }

                        if shift_y >= 0 {
                            win_size.y = (pos_prev_y + shift_y) as u32;
                        } else {
                            let a = shift_y.abs() as i32;
                            win_size.y = if pos_prev_y > a { (pos_prev_y - a) as u32 } else { 0 };
                        }
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        is_down = false;
                    }
                }
                // Обработка прокрутки колесика
                Event::MouseWheel { x: _, y, .. } => {
                    let mut v = win_size.k as i8;
                    v += y as i8;

                    if v < 0 {
                        win_size.k = 0;
                    } else if v >= 16 {
                        win_size.k = 16;
                    } else {
                        win_size.k = v as u8;
                    }
                }
                _ => {}
            }
        }

        // render
        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();

        draw_cell(&mut canvas, &win_size, engine, font);

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for rec in ninja.records.iter() {
            let x = shift + (rec.start as f32 * scale) as i32 - win_size.x as i32;
            let y = shift + ((rec.thread_id) * (win_size.block_h as u32 + win_size.block_delta as u32)) as i32 - win_size.y as i32;
            let w = (rec.dur as f32 * scale) as u32;
            let h = (win_size.block_h) as u32;

            if x > win_size.w as i32 || y > win_size.h as i32 || x + (w as i32) < 0 || y + (h as i32) < 0 {
                continue;
            }

            let mut text_len = rec.cmd.len();
            let text = match rec.cmd.split("/").last() {
                None => rec.cmd.as_ptr(),
                Some(t) => {
                    text_len = t.len();
                    t.as_ptr()
                }
            };

            unsafe {
                let text = ttf::TTF_CreateText(engine, font, text as *const i8, text_len);
                ttf::TTF_DrawRendererText(text, 8.0 + x as f32, y as f32);
                ttf::TTF_DestroyText(text);
            }

            canvas.draw_rect(Rect::new(x, y, w, h)).unwrap();
        }

        canvas.present();

        if iter == 0 {
            println!("render time: {:?}", timer.elapsed());
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    unsafe {
        ttf::TTF_CloseFont(font);
        ttf::TTF_Quit();
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
