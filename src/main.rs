extern crate sdl2;
extern crate rand;

mod instr;
mod memory;
mod display;
mod chip;
mod sdl_interface;

use sdl_interface::run_sdl_interface;
use chip::Chip;

fn main() {

    let mut chip = Chip::new();
    chip.memory.load_font();

    use instr::Instr::*;

    chip.memory.load_program(0x200, &[
        LD_R_B(0, 3),
        LD_R_B(1, 10),
        LD_R_B(2, 0xA),
        LD_F_R(2),
        DRW(0, 1, 5),

        LD_R_B(0, 8),
        LD_R_B(1, 10),
        LD_R_B(2, 0x7),
        LD_F_R(2),
        DRW(0, 1, 5),

        JP(0xFFF),
    ]);
    chip.pc = 0x200;

    run_sdl_interface(&mut chip);
}
