extern crate rand;

pub mod instr;
pub mod memory;
pub mod display;
pub mod chip;

use chip::Chip;

fn main() {
    let mut chip = Chip::new();
    chip.memory.load_font();

    use instr::Instr::*;
    // Calc Fibonacci
    chip.memory.load_program(0x200, &[
        LD_R_B(0, 7),
        LD_R_B(1, 1),
        LD_R_B(2, 0),

        // loop:
        // finish if V0 == 0
        SNE(0, 0),
        JP(0xFFF),

        // V1, V2 = V1 + V2, V1
        LD_R_R(3, 1),
        ADD(1, 2),
        LD_R_R(2, 3),

        // decrement V0
        LD_R_B(3, 1),
        SUB(0, 3),

        // jump to loop
        JP(0x200 + 6),
    ]);

    chip.run_program(0x200).unwrap();
    assert_eq!(chip.v[1], 21);
    assert_eq!(chip.v[2], 13);

    chip.memory.load_program(0x200, &[
        LD_R_B(0, 3),
        LD_R_B(1, 10),
        LD_R_B(2, 0xA),
        LD_F_R(2),
        DRW(0, 1, 5),

        CLS,

        LD_R_B(0, 8),
        LD_R_B(1, 10),
        LD_R_B(2, 0x7),
        LD_F_R(2),
        DRW(0, 1, 5),

        JP(0xFFF),
    ]);

    chip.run_program(0x200).unwrap();
    chip.display.dump();
}
