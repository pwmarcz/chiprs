extern crate rand;

pub mod instr;
pub mod memory;
pub mod display;
pub mod chip;

use chip::Chip;

fn main() {
    let mut chip = Chip::new();
    chip.memory.load_font();
    chip.dump();
}
