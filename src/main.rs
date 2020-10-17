use clap::{App, Arg};

use chip8::Chip8;

struct SdlEmulator {
    w: usize,
    h: usize,
    zoom: usize,
    chip8: Chip8,
}

impl SdlEmulator {
    fn new(w: usize, h: usize, zoom: usize) -> SdlEmulator {
        let mut c = Chip8::new();

        SdlEmulator {
            w,
            h,
            zoom,
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
