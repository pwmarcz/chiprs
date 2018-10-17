mod instr;
use instr::Instr;

mod memory;
use memory::{Memory, MEMORY_SIZE};

mod display;
use display::Display;

const V_SIZE: usize = 0x10;
const STACK_SIZE: usize = 0x10;

struct Chip {
    v: [u8; V_SIZE],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    dt: u8,
    st: u8,

    memory: Memory,
    display: Display,
}

impl Chip {
    fn new() -> Chip {
        Chip {
            v: [0; V_SIZE],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            dt: 0,
            st: 0,

            memory: Memory::new(),
            display: Display::new(),
        }
    }

    fn dump(&self) {
        for i in 0..V_SIZE {
            print!("V{:X} ", i);
        }
        println!("   I    PC  SP  instr");
        for i in 0..V_SIZE {
            print!("{:02X} ", self.v[i]);
        }
        print!("  {:02x}  {:04x}  {:02x}", self.i, self.pc, self.sp);
        if self.pc as usize <= MEMORY_SIZE - 2 {
            let b = self.memory.u16_at(self.pc as usize);
            match Instr::from(b) {
                Some(instr) => print!("  {:?}", instr),
                None => print!("  ????"),
            }
        }
        println!();
        println!();
        for i in 0..STACK_SIZE {
            print!("S[{:X}] ", i);
        }
        println!();
        for i in 0..V_SIZE {
            print!("{:04X} ", self.stack[i]);
        }
        println!();
        println!();
    }

    fn skip(&mut self) {
        self.pc += 2;
    }

    fn step(&mut self) -> Result<(), String> {
        if self.pc % 2 == 1 {
            return Err("PC is not aligned".to_string());
        }
        if self.pc as usize > MEMORY_SIZE - 2 {
            return Err("PC out of bounds".to_string());
        }

        let b = self.memory.u16_at(self.pc as usize);
        match Instr::from(b) {
            Some(instr) => self.run_instr(instr),
            None => Err(format!("could not parse {:04X} as instruction", b)),
        }
    }

    fn run_instr(&mut self, instr: Instr) -> Result<(), String> {
        self.skip();

        use Instr::*;
        match instr {
            CLS => {
                self.display.clear();
            }
            RET => {
                if self.sp == 0 {
                    return Err("stack underflow".to_string());
                }
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }
            SYS(_xyz) => (),
            JP(xyz) => {
                self.pc = xyz;
            }
            JP_V0(xyz) => {
                self.pc = xyz + (self.v[0] as u16);
            }
            CALL(xyz) => {
                if self.sp as usize > STACK_SIZE {
                    return Err("stack overflow".to_string());
                }

                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = xyz;
            }
            SE(x, yz) => {
                if self.v[x as usize] == yz {
                    self.skip();
                }
            }
            SNE(x, yz) => {
                if self.v[x as usize] != yz {
                    self.skip();
                }
            }
            SE_R(x, y) => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.skip();
                }
            }
            SNE_R(x, y) => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.skip();
                }
            }
            LD_R_R(x, y) => {
                self.v[x as usize] = self.v[y as usize];
            }
            LD_R_B(x, yz) => {
                self.v[x as usize] = yz;
            }
            LD_I_A(xyz) => {
                self.i = xyz;
            }
            LD_R_DT(x) => {
                self.v[x as usize] = self.dt;
            }
            // LD_R_K(x) - wait for a keypress
            LD_DT_R(x) => {
                self.dt = self.v[x as usize];
            }
            LD_ST_R(x) => {
                self.st = self.v[x as usize];
            }
            LD_F_R(x) => {
                // check if vx <= 0xF?
                let vx = self.v[x as usize];
                self.i = (vx as u16) * 5;
            }
            LD_B_R(x) => {
                if self.i as usize > MEMORY_SIZE - 3 {
                    return Err("I out of bounds".to_string());
                }
                let vx = self.v[x as usize];
                self.memory.bytes[self.i as usize] = vx / 100;
                self.memory.bytes[(self.i + 1)as usize] = (vx / 10) % 10;
                self.memory.bytes[(self.i + 2) as usize] = (vx / 100) % 10;
            }
            LD_II_R(x) => {
                if self.i as usize > MEMORY_SIZE - 1 {
                    return Err("I out of bounds".to_string());
                }
                self.memory.bytes[self.i as usize] = self.v[x as usize];
            }
            LD_R_II(x) => {
                if self.i as usize > MEMORY_SIZE - 1 {
                    return Err("I out of bounds".to_string());
                }
                self.v[x as usize] = self.memory.bytes[self.i as usize];
            }
            OR(x, y) => {
                self.v[x as usize] |= self.v[y as usize];
            }
            AND(x, y) => {
                self.v[x as usize] &= self.v[y as usize];
            }
            XOR(x, y) => {
                self.v[x as usize] ^= self.v[y as usize];
            }
            ADD(x, y) => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                let (result, bit) = vx.overflowing_add(vy);
                self.v[x as usize] = result;
                self.v[0xF] = bit as u8;
            }
            SUB(x, y) => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                let (result, bit) = vx.overflowing_sub(vy);
                self.v[x as usize] = result;
                self.v[0xF] = (!bit) as u8;
            }
            SUBN(x, y) => {
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                let (result, bit) = vy.overflowing_sub(vx);
                self.v[x as usize] = result;
                self.v[0xF] = (!bit) as u8;
            }
            // RND(x, yz)
            DRW(x, y, z) => {
                let sprite = &self.memory.bytes[(self.i as usize)..((self.i + z as u16) as usize)];
                let collision = self.display.draw(self.v[x as usize] as usize,
                                             self.v[y as usize] as usize,
                                             sprite);
                self.v[0xF] = collision as u8;
            }
            _ => return Err(format!("instruction not implemented: {:?}", instr)),
        }
        Ok(())
    }

    fn run_program(&mut self, addr: usize) -> Result<(), String> {
        self.pc = addr as u16;
        while self.pc != 0xFFF {
            self.step()?;
        }
        Ok(())
    }
}

fn main() {
    let mut chip = Chip::new();
    chip.memory.load_font();

    use Instr::*;
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
