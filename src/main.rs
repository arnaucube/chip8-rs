extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::Canvas;

use clap::{App, Arg};

use chip8::Chip8;

struct SdlEmulator {
    w: usize,
    h: usize,
    zoom: usize,
    canvas: Canvas<sdl2::video::Window>,
    chip8: Chip8,
}

impl SdlEmulator {
    fn new(w: usize, h: usize, zoom: usize) -> SdlEmulator {
        let mut c = Chip8::new();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        SdlEmulator {
            w,
            h,
            zoom,
            canvas,
            chip8: c,
        }
    }
    fn draw_graphics(&mut self) {
        // TODO
    }
    fn set_keys(&mut self) {
        // TODO
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
    loop {
        e.chip8.emulate_cycle();
        if e.chip8.draw_flag {
            e.draw_graphics();
        }
        e.set_keys();
        // delay
    }
}
