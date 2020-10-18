use std::collections::HashMap;
use std::{thread, time};
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use clap::{App, Arg};

use chip8::Chip8;

struct SdlEmulator {
    w: usize,
    h: usize,
    zoom: usize,
    sdl_context: sdl2::Sdl,
    canvas: Canvas<sdl2::video::Window>,
    vkeys: HashMap<Keycode, u8>,
    chip8: Chip8,
}

impl SdlEmulator {
    fn new(w: usize, h: usize, zoom: usize) -> SdlEmulator {
        let mut c = Chip8::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", (w * zoom) as u32, (h * zoom) as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut vkeys: HashMap<Keycode, u8> = HashMap::new();
        vkeys.insert(Keycode::Num1, 0x01);
        vkeys.insert(Keycode::Num2, 0x02);
        vkeys.insert(Keycode::Num3, 0x03);
        vkeys.insert(Keycode::Num4, 0x0c);
        vkeys.insert(Keycode::Q, 0x04);
        vkeys.insert(Keycode::W, 0x05);
        vkeys.insert(Keycode::E, 0x06);
        vkeys.insert(Keycode::R, 0x07);
        vkeys.insert(Keycode::A, 0x08);
        vkeys.insert(Keycode::S, 0x09);
        vkeys.insert(Keycode::D, 0x0E);
        vkeys.insert(Keycode::F, 0x0A);
        vkeys.insert(Keycode::Z, 0x00);
        vkeys.insert(Keycode::X, 0x0B);
        vkeys.insert(Keycode::V, 0x0F);

        SdlEmulator {
            w,
            h,
            zoom,
            sdl_context,
            canvas,
            vkeys,
            chip8: c,
        }
    }
    fn draw_graphics(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for y in 0..self.h {
            for x in 0..self.w {
                let pixel = self.chip8.gfx[y * self.w + x];
                if pixel != 0 {
                    self.canvas.fill_rect(Rect::new(
                        (x * self.zoom) as i32,
                        (y * self.zoom) as i32,
                        (self.zoom) as u32,
                        (self.zoom) as u32,
                    ));
                }
            }
        }

        self.canvas.present();
        self.chip8.draw_flag = false;
    }
    fn set_keys(&mut self) -> Result<(), String> {
        let mut events = self.sdl_context.event_pump()?;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quit");
                    std::process::exit(0);
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    println!("k {:?}", keycode);
                    if self.vkeys.contains_key(&keycode) {
                        let k_hex = self.vkeys.get(&keycode).unwrap();
                        self.chip8.key[*k_hex as usize] = 1;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    println!("k {:?}", keycode);
                    if self.vkeys.contains_key(&keycode) {
                        let k_hex = self.vkeys.get(&keycode).unwrap();
                        self.chip8.key[*k_hex as usize] = 0;
                    }
                    if keycode == Keycode::Escape {
                        println!("EXIT");
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() {
    let matches = App::new("chip8-rs")
        .version("0.0.1")
        .about("chip8 emulator")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("File path of the rom to load"),
        )
        .get_matches();
    let file = matches.value_of("file");
    let file = match file {
        Some(file) => file,
        _ => panic!("Please specify file path of the rom to load"),
    };
    println!("{:?}", file);

    let mut e = SdlEmulator::new(64, 32, 8);
    e.chip8.load_game(file);

    loop {
        e.chip8.emulate_cycle();
        if e.chip8.draw_flag {
            e.draw_graphics();
        }
        e.set_keys();
        std::thread::sleep(time::Duration::from_millis(1000 / 60));
    }
}
