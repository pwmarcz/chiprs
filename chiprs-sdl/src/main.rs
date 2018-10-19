extern crate sdl2;
extern crate chiprs;

mod sdl_interface;

use sdl_interface::run_sdl_interface;
use chiprs::Chip;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            let filename = &args[1];
            run(filename);
        }
        _ => {
            println!("Usage: {} rom.ch8", args[0]);
        }
    }
}

fn run(filename: &str) {
    let mut chip = Chip::new();
    chip.memory.load_font();

    chip.memory.load_program_from_file(0x200, filename).unwrap();
    chip.jump(0x200);

    run_sdl_interface(&mut chip);
}
